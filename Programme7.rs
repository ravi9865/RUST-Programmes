//7. Implement a function that returns the kth smallest element in a given array.
fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}
fn kth_smallest_helper(arr: &mut [i32], k: usize) -> i32 {
    let pivot_index = partition(arr);
    match pivot_index.cmp(&k) {
        std::cmp::Ordering::Equal => arr[pivot_index],
        std::cmp::Ordering::Less => kth_smallest_helper(&mut arr[pivot_index + 1..], k - pivot_index - 1),
        std::cmp::Ordering::Greater => kth_smallest_helper(&mut arr[..pivot_index], k),
    }
}
fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    kth_smallest_helper(arr, k - 1)
}
fn main() {
    let mut arr = [5, 4, 9, 1, 2, 8, 3, 7, 6];
    let k = 3;
    let kth_smallest = kth_smallest(&mut arr, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}
// complexity -> O(n)