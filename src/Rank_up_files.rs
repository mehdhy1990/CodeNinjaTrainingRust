pub fn create_phone_number(numbers: &[u8]) -> String {
    let strings = numbers.iter().map(|x| x.to_string()).collect::<String>();
    format!("({}) {}-{}", &strings[..3], &strings[3..6], &strings[6..])
}

pub fn persistence(num: u64) -> u64 {
    let mut number = num;
    let mut count = 0;
    while number >= 10 {
        number = number
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .into_iter()
            .fold(1, |acc, x| acc * x);
        count += 1;
    }

    count
}
pub fn dig_pow(n: i64, p: i32) -> i64 {
    let r: i64 = n
        .to_string()
        .chars()
        .map(|c| (c as i64) - 48)
        .enumerate()
        .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
        .sum();

    match r % n {
        0 => r / n,
        _ => -1,
    }
}