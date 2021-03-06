pub fn naive_parallel_count_ones_16(experiment: &Vec<u16>) -> Vec<u16> {
    let m = [
        0b0101010101010101u16,
        0b0011001100110011u16,
        0b0000111100001111u16,
        0b0000000011111111u16,
        0b1111111111111111u16,
        0b1111111111111111u16,
        0b1111111111111111u16,
        0b1111111111111111u16,
    ];
    let mut set = experiment.clone();
    let log_d = 4;
    for i in 0..log_d {
        for word in set.iter_mut() {
            *word = (*word & m[i]) + ((*word >> (1 << i)) & m[i])
        }
    }
    set
}
pub fn naive_parallel_count_ones_32(experiment: &Vec<u32>) -> Vec<u32> {
    let m = [
        0b01010101010101010101010101010101u32,
        0b00110011001100110011001100110011u32,
        0b00001111000011110000111100001111u32,
        0b00000000111111110000000011111111u32,
        0b00000000000000001111111111111111u32,
        0b11111111111111111111111111111111u32,
        0b11111111111111111111111111111111u32,
        0b11111111111111111111111111111111u32,
    ];
    let mut set = experiment.clone();
    let log_d = 6;
    for i in 0..log_d {
        for word in set.iter_mut() {
            *word = (*word & m[i]) + ((*word >> (1 << i)) & m[i])
        }
    }
    set
}
pub fn naive_parallel_count_ones_64(experiment: &Vec<u64>) -> Vec<u64> {
    let m = [
        0b0101010101010101010101010101010101010101010101010101010101010101u64,
        0b0011001100110011001100110011001100110011001100110011001100110011u64,
        0b0000111100001111000011110000111100001111000011110000111100001111u64,
        0b0000000011111111000000001111111100000000111111110000000011111111u64,
        0b0000000000000000111111111111111100000000000000001111111111111111u64,
        0b0000000000000000000000000000000011111111111111111111111111111111u64,
        0b1111111111111111111111111111111111111111111111111111111111111111u64,
    ];
    let mut set = experiment.clone();
    let log_d = 6;
    for i in 0..log_d {
        for word in set.iter_mut() {
            *word = (*word & m[i]) + ((*word >> (1 << i)) & m[i])
        }
    }
    set
}
pub fn naive_parallel_count_ones_128(experiment: &Vec<u128>) -> Vec<u128> {
    let m = [0b01010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101u128,
0b00110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011u128,
0b00001111000011110000111100001111000011110000111100001111000011110000111100001111000011110000111100001111000011110000111100001111u128,
0b00000000111111110000000011111111000000001111111100000000111111110000000011111111000000001111111100000000111111110000000011111111u128,
0b00000000000000001111111111111111000000000000000011111111111111110000000000000000111111111111111100000000000000001111111111111111u128,
0b00000000000000000000000000000000111111111111111111111111111111110000000000000000000000000000000011111111111111111111111111111111u128,
0b00000000000000000000000000000000000000000000000000000000000000001111111111111111111111111111111111111111111111111111111111111111u128,
    ];
    let mut set = experiment.clone();
    let log_d = 7;
    for i in 0..log_d {
        for word in set.iter_mut() {
            *word = (*word & m[i]) + ((*word >> (1 << i)) & m[i])
        }
    }
    set
}
#[test]
fn test_random_64() {
    use rand::Rng;
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let mut val: Vec<u64> = Vec::with_capacity(1 << 14);
        for _ in 0..(1 << 14) {
            val.push(rng.gen::<u64>());
        }
        let expected: Vec<u64> = val.iter().map(|x| x.count_ones() as u64).collect();
        let res = naive_parallel_count_ones_64(&val);
        assert_eq!(res.len(), expected.len());
        for i in 0..val.len() {
            assert_eq!(res[i], expected[i]);
        }
    }
}
#[test]
fn test_random_128() {
    use rand::Rng;
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let mut val: Vec<u128> = Vec::with_capacity(1 << 14);
        for _ in 0..(1 << 14) {
            val.push(rng.gen::<u128>());
        }
        let expected: Vec<u128> = val.iter().map(|x| x.count_ones() as u128).collect();
        let res = naive_parallel_count_ones_128(&val);
        assert_eq!(res.len(), expected.len());
        for i in 0..val.len() {
            assert_eq!(res[i], expected[i]);
        }
    }
}
