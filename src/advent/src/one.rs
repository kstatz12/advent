//use crate::reader::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> i32 {
    let filename = "src/input/one_input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut inputs : Vec<i32> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let i = line.parse::<i32>().unwrap();
        inputs.push(i);
    }
    sum(inputs)
}

fn calc(mass : i32) -> i32 {
    let f = (mass as f64 / 3.0).floor();
    f as i32 - 2
}

fn sum(inputs : Vec<i32>) -> i32 {
    let mut total : i32 = 0;
    for i in inputs.iter(){
        let r = calc(*i);
        total += r;
    }
    total
}

