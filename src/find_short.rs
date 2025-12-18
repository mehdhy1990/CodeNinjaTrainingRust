fn find_short(s: &str) -> u32 {
    //your code here
    let x = s.split_whitespace().min_by_key(|x| x.len()).unwrap().len();
    x as u32
}