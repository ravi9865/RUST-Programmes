//5. Given a sorted array of integers, implement a function that returns the median of the array.
fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        return (arr[mid - 1] + arr[mid]) as f64 / 2.0;
    } else {
        let mid = len / 2;
        return arr[mid] as f64;
    }
}
fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let median = find_median(&sorted_array);
    println!("Median: {}", median);
}
// complexity -> O(1)