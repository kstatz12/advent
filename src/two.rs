use crate::{
    parser::get_two_input,
    computer::*,
};

pub fn run() -> i32 {
    let mut inputs = get_two_input("src/input/two_input.txt");
    //as per directions
    inputs[1] = 12;
    inputs[2] = 2;
    let r = solve(0, &mut inputs);
    r[0]
}


