//9. Reverse a string in Rust
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn main() {
    let original = "hello";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
// complexity -> O(n)