pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    let sounds = ["Pling", "Plang", "Plong"];

    for (i, divisor) in [3, 5, 7].iter().enumerate() {
        if n % divisor == 0 {
            result.push_str(sounds[i]);
        }
    }

    if result.is_empty() {
        result = n.to_string();
    }

    result
}
