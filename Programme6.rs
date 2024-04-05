//6. Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first_string = &strings[0];
    let mut prefix = String::new();
    'outer: for (i, c) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(char) = string.chars().nth(i) {
                if char != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }
    prefix
}
fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", common_prefix);
}

// complexity -> O(mn)