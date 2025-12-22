use std::arch::x86_64::_fxrstor64;
use std::collections::HashMap;

mod rank_up_files;

fn main() {
    let result = rank_up_files::solution(11);

    println!("{} this is the result", result)
}

// fn find_uniq(arr: &[f64]) -> f64 {
//     let mut counts = HashMap::new();
//     for num in arr {
//         counts.entry(num).or_insert(0.0) += 1;
//     }
//     todo!()
// }
