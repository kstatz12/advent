use crate::{
    parser::get_two_input,
    computer::*,

};

use std::convert::TryInto;

pub fn run() -> i32 {
    let mut res = 0;
    for lhs in 0..99 {
        for rhs  in 0..99{
                let mut inputs = get_two_input("src/input/two_input.txt");
                inputs[1] = lhs.try_into().unwrap();
                inputs[2] = rhs.try_into().unwrap();

                let r = solve(0, &mut inputs);
                if r[0] == 19690720 {
                    res = 100 * lhs + rhs;
                }
        }
    }
    res
}
