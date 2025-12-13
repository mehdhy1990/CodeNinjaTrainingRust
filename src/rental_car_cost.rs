pub fn rental_car_cost(d: u32) -> u32 {
    match d {
        c if  c > 3 && c<7 => c*40 -20,
        c if c >= 7 => c*40 -50,
        _ => d *40,

    }
}