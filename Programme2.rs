// 2.Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start <= end {
        let mid = start + (end - start) / 2;
        if arr[mid] == target {
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                end = mid - 1;
            }
        } else if arr[mid] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    None
}
fn main() {
    let arr = vec![1, 2, 2, 3, 3, 3, 4, 5, 5, 5, 6];
    let target = 3;
    match first_occurrence(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
// complexity -> O(logn)