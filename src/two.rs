use crate::{
    parser::get_two_input,
    calc::*,
};

pub fn run() -> i32 {
    let mut inputs = get_two_input("src/input/two_input.txt");
    //as per directions
    inputs[1] = 12;
    inputs[2] = 2;
    let r = solve(0, &mut inputs);
    r[0]
}


fn solve(start_idx : usize, vect : &mut Vec<i32>) -> Vec<i32> {
    let opcode = vect[start_idx];

    let left : usize = start_idx + 1;
    let right : usize = start_idx + 2;
    let output : usize = start_idx + 3;

    let r = match opcode {
        1 => add(left, right, vect),
        2 => multiply(left, right, vect),
        99 => -1,
        _ => -2,
    };

    if r == -1 {
        vect.to_vec()
    }
    else {
        let output_pos = vect[output] as usize;
        vect[output_pos] = r;
        solve(start_idx + 4, vect)
    }
}



