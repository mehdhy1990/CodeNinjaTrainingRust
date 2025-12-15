pub fn grow(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(1, |sum, x| sum * x)

}