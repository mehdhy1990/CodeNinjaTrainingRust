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