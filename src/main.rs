mod are_you_playing_banjo;
mod double_char;
mod positive_sum;
mod abbrev_name;
mod grow;
mod open_or_senior;

fn main() {
let vec:Vec<(i32,i32)> = vec![(18, 20), (45, 2), (61, 12), (37, 6), (21, 21), (78, 9)];
    let result =open_or_senior::open_or_senior(vec);
    println!("{:#?}", result);
}

