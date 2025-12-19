use std::ffi::c_int;

mod Rank_up_files;
mod Fundamentals;

fn main() {
    let number = vec!['a', 'b', 'd', 'e', 'f', 'g'];
    let result = find_missing_letter(&number);
    let mut number2 = 'a' as u8;
    println!("{} this is the result", result);
}

fn find_missing_letter(chars: &[char]) -> char {
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
