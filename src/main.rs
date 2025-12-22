mod rank_up_files;

fn main() {
    let result = rank_up_files::spin_words("Hey fellow warriors");

    println!("{} this is the result", result)
}

// fn find_uniq(arr: &[f64]) -> f64 {
//     let mut counts = HashMap::new();
//     for num in arr {
//         counts.entry(num).or_insert(0.0) += 1;
//     }
//     todo!()
// }
fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| {
            match c {
                'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
                'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
                _ => c, // Keep numbers, spaces, and symbols as they are
            }
        })
        .collect()
}
