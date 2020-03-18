pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut prev_index = 0;
    let all_for_object = list[prev_index];
    let mut lines = vec![];

    for (i, s) in list.iter().enumerate() {
        if i > prev_index {
            lines.push(format!(
                "For want of a {} the {} was lost.",
                list[prev_index], s
            ));
            prev_index += 1;
        }
    }

    lines.push(format!("And all for the want of a {}.", all_for_object));
    lines.join("\n")
}
