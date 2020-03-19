pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for incoming in string.chars() {
        let last = stack.last();
        match (last, incoming) {
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => {
                stack.pop();
            }
            (_, ')') | (_, ']') | (_, '}') => {
                return false;
            }
            (_, '(') | (_, '[') | (_, '{') => {
                stack.push(incoming);
            }
            _ => (),
        }
    }

    stack.is_empty()
}
