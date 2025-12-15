pub fn repeat_str(src: &str, count: usize) -> String {
    let mut s = String::new();
    for _i in 0..count {
        s.push_str(src)
    }
    return s;

    //src.repeat(count)
}