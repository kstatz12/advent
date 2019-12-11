use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_ints(filename : &'static str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut inputs : Vec<i32> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let i = line.parse::<i32>().unwrap();
        inputs.push(i);
    }
    inputs
}

pub fn get_two_input(filename : &'static str) -> mut& Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let 
}

