use crate::{
    parser::*,
    calc::*,
};


pub fn run() -> i32 {
    let inputs = get_ints("src/input/one_input.txt");
    sum(inputs)
}

fn sum(inputs : Vec<i32>) -> i32 {
    let mut total : i32 = 0;
    for i in inputs.iter(){
        let r = calculate_fuel(*i);
        total += r;
    }
    total
}

