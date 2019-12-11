pub fn calculate_fuel(mass : i32) -> i32 {
    let f = (mass as f64 / 3.0).floor();
    f as i32 - 2
}

