

mod rank_up_files;


fn main() {
    let number = vec!['a', 'b', 'd', 'e', 'f', 'g'];
    let result = rank_up_files::find_missing_letter(&number);

    println!("{} this is the result", result);
}

