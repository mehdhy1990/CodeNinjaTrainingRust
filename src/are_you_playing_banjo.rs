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