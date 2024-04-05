// 1.Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();
    let reversed = input.chars().rev().collect::<String>();
    input == reversed 
}
fn main() {
    let arr = vec!["racecar", "hello", "level", "A man, a plan, a canal, Panama"];
    for val in arr {
        println!("Is '{}' a palindrome? {}", val, is_palindrome(val));
    }
}
// complexity -> O(n)