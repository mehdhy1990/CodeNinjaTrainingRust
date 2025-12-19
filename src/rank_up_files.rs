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
pub fn square_digits(num: u64) -> u64 {
    let mut n = num;
    let mut digits: Vec<u64> = Vec::new();

    while n > 0 {
        // 1. Get the last digit using modulo (911 % 10 = 1)
        let digit = n % 10;
        digits.push(digit);

        // 2. Remove the last digit using division (911 / 10 = 91)
        n /= 10;
    }
    let result=   digits.iter().rev().map(|d| (d*d).to_string()).collect::<Vec<_>>().join("").parse::<u64>();
    result.unwrap()
}
pub fn find_missing_letter(chars: &[char]) -> char {
    let vec = chars.iter().map(|&x| x as u8).collect::<Vec<u8>>();
    let result = vec.windows(2).find(|x| x[0] + 1 != x[1]).unwrap();
    (result[0] + 1) as char


    //best practice
    // -----------------------------------------
    // (chars.windows(2)                        //Convert into a list of arrays length 2, for ease of viewing past and next character.
    //     .map(|w| (w[0] as u8, w[1] as u8)) //Convert the character array to a byte tuple.
    //     .find(|&w| w.0 + 1 != w.1)         //Example: 'a' + 1 != 'b'? Found a missing character.
    //     .unwrap().0 + 1) as char           //Add 1 to previous character, to get the correct character.

}
