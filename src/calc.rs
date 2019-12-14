pub fn calculate_fuel(mass : i32) -> i32 {
    let f = (mass as f64 / 3.0).floor();
    f as i32 - 2
}

pub fn add(lhs : usize, rhs : usize, vect : &Vec<i32>) -> i32 {
    let lhs_val = vect[lhs];
    let rhs_val = vect[rhs];
    lhs_val + rhs_val 
}

pub fn multiply(lhs : usize, rhs : usize, vect: &Vec<i32>) -> i32{
    let lhs_val = vect[lhs];
    let rhs_val = vect[rhs];
    lhs_val * rhs_val 
}



