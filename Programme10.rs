// 10. Check if a number is prime in Rust
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    let mut is_prime = true;
    let mut divisor = 2;
    while divisor * divisor <= num {
        if num % divisor == 0 {
            is_prime = false;
            break;
        }
        divisor += 1;
    }
    is_prime
}
fn main() {
    let number = 29;
    if is_prime(number) {
        println!("{} is prime", number);
    } else {
        println!("{} is not prime", number);
    }
}
// complexity -> O(√n​)