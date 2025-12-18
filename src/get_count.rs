pub fn get_count(string: &str) -> usize {
    let vowels_count: usize;
    vowels_count = string
        .chars()
        .filter(|c| match *c {
            'a' => true,
            'e' => true,
            'i' => true,
            'o' => true,
            'u' => true,
            _ => false,
        })
        .count();
    return vowels_count;
}