/// Split string on specified delimiter, but only on surface level <br>
/// Also removes all spaces
pub fn split_surface(string: &str, delimiter: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut depth = 0;

    let string = string.replace(" ", "");

    for (i, c) in string.chars().enumerate() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ if c == delimiter && depth == 0 => {
                result.push(string[start..i].to_string());
                start = i + 1;
            },
            _ => {}
        }
    }
    result.push(string[start..].to_string());

    result
}