/// grains of rice on square `s`
pub fn square(s: u32) -> u64 {
    match s {
        s @ 1..=64 => 1u64 << s - 1,
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum::<u64>()
}
