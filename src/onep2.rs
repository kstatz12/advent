use crate::{
    parser::*,
    calc::*,
};

pub fn run() -> i64 {
    let inputs = get_ints("src/input/one_input.txt");
    sum(inputs)
}

fn sum(inputs : Vec<i32>) -> i64 {
    let mut total : i64 = 0;
    for i in inputs.iter(){
        let mut inner_total : i32 = 0;
        let r = recurse(*i, &mut inner_total) as i64;
        total += r;
    }
    total
}

fn recurse(input : i32, total : &mut i32) -> i32{
    let r = calculate_fuel(input);
    if r <= 0 {
       *total 
    } else {
        *total += r;
        recurse(r, total)
    }
}


