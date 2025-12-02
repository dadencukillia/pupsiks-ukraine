/// Trims a string, removes all control characters, and collapses multiple spaces into a single space, resulting in a clean and standardized string
pub fn smart_trim(input: &str) -> String {
    let trimmed_and_cleaned: String = input
        .trim()
        .chars()
        .filter(|c| !c.is_control())
        .collect();

    let mut result = String::with_capacity(trimmed_and_cleaned.len());
    let mut last_char_was_space = false;

    for c in trimmed_and_cleaned.chars() {
        if c.is_whitespace() {
            if !last_char_was_space {
                result.push(' ');
                last_char_was_space = true;
            }
        } else {
            result.push(c);
            last_char_was_space = false;
        }
    }

    result.trim().to_string()
}
