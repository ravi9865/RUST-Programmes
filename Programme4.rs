//4. Implement a function that checks whether a given number is prime or not.
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
fn main() {
    let numbers = vec![2, 3, 5, 7, 11, 12, 13, 17, 19, 23, 29, 31];
    for num in numbers {
        println!("Is {} prime? {}", num, is_prime(num));
    }
}
// complexity -> O(√n)