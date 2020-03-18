pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    // If it ends with question mark
    let a_question = trimmed.ends_with("?");

    // If it contains any letter in the alphabet
    let has_letter = trimmed.contains(char::is_alphabetic);

    // If it contains any letter and all letters are in uppercase
    let a_yell = has_letter && trimmed == trimmed.to_ascii_uppercase();

    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if a_yell && a_question {
        "Calm down, I know what I'm doing!"
    } else if a_yell {
        "Whoa, chill out!"
    } else if a_question {
        "Sure."
    } else {
        "Whatever."
    }
}
