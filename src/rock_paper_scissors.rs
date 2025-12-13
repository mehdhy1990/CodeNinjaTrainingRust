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