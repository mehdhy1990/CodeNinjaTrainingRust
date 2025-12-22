
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
