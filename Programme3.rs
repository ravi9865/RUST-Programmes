// 3.Given a string of words, implement a function that returns the shortest word in the string.
fn shortest_word(input: &str) -> String {
    let mut shortest = String::new();
    let mut word = String::new();
    let mut is_word = false;
    for ch in input.chars() {
        if ch.is_alphabetic() {
            is_word = true;
            word.push(ch);
        } else if is_word {
            if shortest.is_empty() || word.len() < shortest.len() {
                shortest = word.clone();
            }
            word.clear();
            is_word = false;
        }
    }
    if is_word && (shortest.is_empty() || word.len() < shortest.len()) {
        shortest = word;
    }
    shortest
}
fn main() {
    let input = "The quick brown fox jumps over the lazy dog";
    println!("Shortest word: {}", shortest_word(input));
}
// complexity -> O(n)