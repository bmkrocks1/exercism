/// square of sum of 1...n
pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

/// sum of squares of 1...n
pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).sum::<u32>()
}

/// difference between square of sum of 1...n and sum of squares of 1...n
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
