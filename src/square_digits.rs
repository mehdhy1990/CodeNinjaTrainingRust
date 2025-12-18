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