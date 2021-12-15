use std::ops::Add;
use std::ops::BitAnd;
use std::ops::Not;
use std::ops::Shl;
use std::ops::Shr;

pub enum WordLength {
    U64,
    U128,
}

pub trait Word:
    Add<Output = Self>
    + BitAnd<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Not<Output = Self>
    + Copy
where
    Self: std::marker::Sized,
{
    fn zero() -> Self;
}
impl Word for u64 {
    fn zero() -> u64 {
        0
    }
}
impl Word for u128 {
    fn zero() -> u128 {
        0
    }
}

#[inline(always)]
fn isolate_blocks<T: Word>(word: &T, mask: &T, shift_by: usize) -> T {
    (*word >> shift_by) & *mask
}

#[inline(always)]
fn k_prime<T: Word>(k: &T, i: usize, mask: &T) -> T {
    isolate_blocks(k, mask, 0) + isolate_blocks(k, mask, 1 << i)
}

#[inline(always)]
fn combine_words<T: Word>(word1: T, word2: T, i: usize) -> T {
    word1 + (word2 << (1 << i))
}

#[inline(always)]
fn calculate_l(i: usize) -> usize {
    // The "proper" way to do this is to do ceil(log_2(i + 2)), but we hardcode it to save the log_2 computation
    // On 64-bit words, i will never exceed 7
    match i + 2 {
        2 => 1,
        3..=4 => 2,
        5..=8 => 3,
        9..=16 => 4,
        16..=32 => 5,
        x => (x as f64).log2().ceil() as usize,
    }
}

#[inline(always)]
fn pack_word<T: Word>(word: &T, i: usize, mask: &T) -> T {
    let l = calculate_l(i);
    isolate_blocks(word, mask, 0) + (isolate_blocks(word, mask, 1 << l) << (1 << i))
}

#[inline(always)]
fn naive_pack_word<T: Word>(word: &T, i: usize, mask: &T) -> T {
    isolate_blocks(word, mask, 0) + isolate_blocks(word, mask, 1 << i)
}

pub fn count_ones<T: Word>(experiment: &Vec<T>, masks: [[T; 8]; 8]) -> Vec<T> {
    let log_d = 6; // log_2(64) = 6
    let mut set = experiment.clone();
    // first use the naive algorithm for two steps. This part takes O(m) time.
    for k in &mut set {
        *k = naive_pack_word(&k, 0, &masks[0][1]);
        *k = naive_pack_word(&k, 1, &masks[1][2]);
    }
    for i in 2..log_d {
        for k in &mut set {
            *k = k_prime(&k, i, &masks[i][i + 1]); // each k is now prime and (i)-packed
        }
        let l = calculate_l(i);
        if l == calculate_l(i + 1) {
            for k in 0..(set.len() / 2) {
                set[k] = combine_words(set[k << 1], set[(k << 1) + 1], i); // each k is now (i+1)-packed
            }
            set.truncate(set.len() / 2) // truncate is a constant-time operation
        } else {
            for word in &mut set {
                *word = pack_word(&word, i, &masks[l][l + 1]); // each k is now (i+1)-packed
            }
        }
    }

    // Make a vector containing the cardinalities of each element
    let mut acc = Vec::with_capacity(set.len());
    let l = calculate_l(log_d);
    for word in set {
        for k in 0..(1 << l) {
            acc.push(isolate_blocks(&word, &masks[l][log_d], k << l));
        }
    }
    acc
}

#[test]
fn test_random_64() {
    use crate::calculate_mask::GetMask;
    use rand::Rng;
    let masks = u64::get_mask();
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let mut val: Vec<u64> = Vec::with_capacity(1 << 14);
        for _ in 0..(1 << 14) {
            val.push(rng.gen::<u64>());
        }
        let expected: Vec<u64> = val.iter().map(|x| x.count_ones() as u64).collect();
        let res = count_ones(&val, masks);
        assert_eq!(res.len(), expected.len());
        for i in 0..val.len() {
            assert_eq!(res[i], expected[i]);
        }
    }
}
