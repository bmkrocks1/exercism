fn get_digits(mut n: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let len = digits.len() as u32;
    let sum = digits.iter().map(|d| d.pow(len)).sum::<u32>();
    sum == num
}
