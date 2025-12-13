pub fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let product_a = a.iter().fold(1, |acc, &x| acc * x);
    let product_b = b.iter().fold(1, |acc, &x| acc * x);
    (product_a - product_b).abs()
}