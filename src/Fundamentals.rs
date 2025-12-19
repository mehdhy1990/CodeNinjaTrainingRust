pub fn abbrev_name(name: &str) -> String {
    name.split_ascii_whitespace().into_iter().map(|x| &x[..1]).collect::<Vec<_>>().join(".")
}
pub fn are_you_playing_banjo(name: &str) -> String {
    // match name {
    //     x if x.starts_with('R')=>{
    //         format!("{} plays banjo ", name)
    //     }
    //     x if x.starts_with('r')=>{
    //         format!("{} plays banjo ", name)
    //     }
    //     _ => format!("{} does not play banjo", name)
    // }



    //this is the best practice
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}
fn dna_strand(dna: &str) -> String {
    // let mut scores:HashMap<char,char> = HashMap::new();
    // scores.insert('A','T');
    // scores.insert('T','A');
    // scores.insert('C','G');
    // scores.insert('G','C');
    // dna.chars().map(|x| {
    //     if(scores.contains_key(&x)) {
    //         return scores[&x];
    //     }
    //    return  x;
    // }).collect::<String>()
    dna.chars()
        .map(|x| match x {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            s => s,
        })
        .collect::<String>()
}
pub fn double_char(s: &str) -> String {
    let ret = s.chars().map(|c| format!("{}{}", c, c))
        .collect();
    ret
    // let result = s.chars().flat_map(|c| [c, c]).collect()
    // return result;
}
pub fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let product_a = a.iter().fold(1, |acc, &x| acc * x);
    let product_b = b.iter().fold(1, |acc, &x| acc * x);
    (product_a - product_b).abs()
}
pub fn find_short(s: &str) -> u32 {
    //your code here
    let x = s.split_whitespace().min_by_key(|x| x.len()).unwrap().len();
    x as u32
}
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
pub fn grow(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(1, |sum, x| sum * x)

}
pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.iter().map(|&(x,y)| {
        if x >55 && y>7 {
            return "Senior".to_string();
        }
        return  "Open".to_string();
    }).collect::<Vec<String>>()

    // data.iter().map(|&(x, y)| match (x >= 55,y > 7) {
    //     (true, true) => String::from("Senior"),
    //     _ => String::from("Open")}).collect()
    //this is one best solutions
}
fn positive_sum(slice: &[i32]) -> i32 {
    // your code
    slice.iter().filter(|&&x| x > 0).sum()
}
pub fn  rps(p1: &str, p2: &str) -> &'static str {
    match p1 {
        "scissors" => match p2 {
            "paper" => "Player 1 won!",
            "rock" => "Player 2 won!",
            _ => ("Draw!"),
        },
        "paper" => match p2 {
            "scissors" => "Player 2 won!",
            "rock" => "Player 1 won!",
            _ => "Draw!",
        }
        "rock" => match p2 {
            "paper" => "Player 2 won!",
            "scissors" => "Player 1 won!",
            _ => "Draw!",
        }
        _ => "enter valid input",
    }
}
pub fn best_rps(p1: &str, p2: &str) -> &'static str {
    //this is the best solution and clearly explained itself
    if (p1 == p2) {
        return "Draw!";
    }
    match (p1, p2) {
        ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
        _ => "Player 2 won!",
    }
}
pub fn repeat_str(src: &str, count: usize) -> String {
    let mut s = String::new();
    for _i in 0..count {
        s.push_str(src)
    }
    return s;

    //src.repeat(count)
}
pub fn rental_car_cost(d: u32) -> u32 {
    match d {
        c if  c > 3 && c<7 => c*40 -20,
        c if c >= 7 => c*40 -50,
        _ => d *40,

    }
}