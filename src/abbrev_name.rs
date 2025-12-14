fn abbrev_name(name: &str) -> String {
    name.split_ascii_whitespace().into_iter().map(|x| &x[..1]).collect::<Vec<_>>().join(".")
}