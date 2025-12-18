

fn main() {
    let result = create_phone_number(&[1,2,3,4,5,6,7,8,9,0]);
    println!("{}", result);
}

fn create_phone_number(numbers: &[u8]) -> String {
  let strings=  numbers.iter().map(|x| x.to_string()).collect::<String>();
    format!("({}) {}-{}",&strings[..3],&strings[3..6],&strings[6..])

}