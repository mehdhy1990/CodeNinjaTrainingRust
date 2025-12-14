fn positive_sum(slice: &[i32]) -> i32 {
    // your code
    slice.iter().filter(|&&x| x > 0).sum()
}