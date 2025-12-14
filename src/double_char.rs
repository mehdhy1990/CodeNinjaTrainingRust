fn double_char(s: &str) -> String {
    let ret = s.chars().map(|c| format!("{}{}", c, c))
        .collect();
    ret
    // let result = s.chars().flat_map(|c| [c, c]).collect()
    // return result;
}