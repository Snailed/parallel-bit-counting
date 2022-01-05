use crate::calculate_mask::get_basic_masks_u128;
use core::fmt::Binary;
use core::fmt::Debug;
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
    + Debug
    + Eq
    + Binary
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
    *word >> shift_by & *mask
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
#[test]
fn test_calculate_l() {
    for i in 0..100 {
        assert_eq!(
            calculate_l(i),
            ((i + 2) as f64).log2().ceil() as usize,
            "Failed at iteration i:{}",
            i
        );
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

pub fn count_ones<T: Word>(
    experiment: &Vec<T>,
    masks: [[T; 8]; 8],
    word_length: WordLength,
) -> Vec<T> {
    let log_d = match word_length {
        WordLength::U64 => 6,
        WordLength::U128 => 7,
    }; // log_2(64) = 6
    let mut set = experiment.clone();
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
            for k in 0..(set.len() >> 1) {
                set[k] = combine_words(set[k << 1], set[(k << 1) + 1], i); // each k is now (i+1)-packed
            }
            set.truncate(set.len() >> 1) // truncate is a constant-time operation
        } else {
            for word in &mut set {
                *word = pack_word(&word, i, &masks[l][l + 1]); // each k is now (i+1)-packed
            }
        }
    }

    // Make a vector containing the cardinalities of each element
    let mut acc = Vec::with_capacity(set.len());
    let l = calculate_l(log_d);
    // println!("Set: {:?}", set);
    // For each word in the set
    for word in set {
        // For each cardinality that this word contains
        // (a log_d packed word contains 2^(log_d - l(log_d)) words)
        for k in 0..(1 << (log_d - l)) {
            // Push that exact cardinality to a list
            acc.push(isolate_blocks(&word, &masks[l][log_d], k << l));
        }
    }
    acc
}

#[test]
fn test_specific_128() {
    use crate::calculate_mask::GetMask;
    let input = [
        0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128,
        0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128, 0b1u128,
    ];
    let res = count_ones(&Vec::from(input), u128::get_mask(), WordLength::U128);
    for i in 0..input.len() {
        assert_eq!(input[i], res[i]);
    }
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
        let res = count_ones(&val, masks, WordLength::U64);
        assert_eq!(res.len(), expected.len());
        for i in 0..val.len() {
            assert_eq!(
                res[i], expected[i],
                "left: {:b},\nright: {:b},\n i: {}",
                res[i], expected[i], i
            );
        }
    }
}
#[test]
fn test_small_sample_random_128() {
    use crate::calculate_mask::GetMask;
    let masks = u128::get_mask();
    let val: Vec<u128> = vec![
        225038453427468444376840238419353982120,
        126200853859186040732578121884242145784,
        10262100669963364223699525567592166329,
        294425819082398123114249317675861336733,
        115777319561375603372569018363930472932,
        250901836470375547477895771691870755784,
        18068537806635936277815686070558335203,
        56513041384264465694799984113465250286,
        234390566789637499012445399682518579939,
        262161782149154646596474419290825803878,
        43083132496328971009218099960341993064,
        7892324528197330420472287502510733544,
        186881521432731296949434670679583391062,
        226592890537627480141395410248135930110,
        151200256495250426677114264341598409570,
        227874780865315671664495445171902494754,
    ];
    // println!("input 0 ({} ones): {:b}", val[0].count_ones(), val[0]);
    // println!("input 1 ({} ones): {:b}", val[1].count_ones(), val[1]);
    let expected: Vec<u128> = val.iter().map(|x| x.count_ones() as u128).collect();
    // println!("Expected: {:?}", expected);
    let res = count_ones(&val.clone(), masks, WordLength::U128);
    // println!("Res: {:?}", res);
    assert_eq!(res.len(), expected.len());
    for i in 0..val.len() {
        let res_index = match i & 1 {
            0 => (i >> 1) + ((i >> 3) << 2),
            1 => (i >> 1) + 4 + ((i >> 3) << 2),
            _ => panic!("i%2 gave something illegal!"),
        };
        assert_eq!(
            res[res_index], expected[i],
            "input: {}, left: {:b},\nright: {:b},\n i: {}",
            val[i], res[res_index], expected[i], i
        );
    }
}
#[test]
fn test_random_128() {
    use crate::calculate_mask::GetMask;
    use rand::Rng;
    let masks = u128::get_mask();
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let mut val: Vec<u128> = Vec::with_capacity(1 << 14);
        for _ in 0..(1 << 14) {
            val.push(rng.gen::<u128>());
        }
        let expected: Vec<u128> = val.iter().map(|x| x.count_ones() as u128).collect();
        let res = count_ones(&val.clone(), masks, WordLength::U128);
        assert_eq!(res.len(), expected.len());
        for i in 0..val.len() {
            let res_index = match i & 1 {
                0 => (i >> 1) + ((i >> 3) << 2),
                1 => (i >> 1) + 4 + ((i >> 3) << 2),
                _ => panic!("i%2 gave something illegal!"),
            };
            assert_eq!(
                res[res_index], expected[i],
                "input: {}, left: {:b},\nright: {:b},\n i: {}",
                val[i], res[res_index], expected[i], i
            );
        }
    }
}
