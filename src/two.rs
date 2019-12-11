use crate::{
    parser::*,
    calc::*,
};

pub fun run() {
    
}


pub fn solve(slice : &[i32], vect : mut& Vec<i32>) -> Vec<i32> {
    let opcode = slice[0];

    let r = match opcode {
        1 => add(slice[1], slice[2], vect),
        2 => multiply(slice[1], slice[2], vect),
        _ => -1,
    }

    if r == -1 {
        vect
    }
    else {
        let output_pos = slice[3];
        vect[output_pos] = r;
        vect
    }
}

fn add(lhs : i32, rhs : i32, vect : mut& Vec<i32>) -> i32 {
    let lhsVal = vect[lhs];
    let rhsVal = vect[rhs];
    lhsVal + rhsVal 
}

fn multiply(lhs : i32, rhs : i32, vect: mut& Vec<i32>) -> i32 {
    let lhsVal = vect[lhs];
    let rhsVal = vect[rhs];
    lhsVal * rhsVal 
}
