pub fn naive_count_bits_64(experiment: &mut Vec<u64>) -> &mut Vec<u64> {
    for word in experiment.iter_mut() {
        let mut sum = 0;
        for i in 0..64 {
            sum += (*word >> i) & 1
        }
        *word = sum
    }
    experiment
}
pub fn naive_count_bits_128(experiment: &mut Vec<u128>) -> &mut Vec<u128> {
    for word in experiment.iter_mut() {
        let mut sum = 0;
        for i in 0..128 {
            sum += (*word >> i) & 1
        }
        *word = sum
    }
    experiment
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
        let res = naive_count_bits_64(&mut val);
        assert_eq!(res.len(), expected.len());
        for i in 0..res.len() {
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
        let res = naive_count_bits_128(&mut val);
        assert_eq!(res.len(), expected.len());
        for i in 0..res.len() {
            assert_eq!(res[i], expected[i]);
        }
    }
}
