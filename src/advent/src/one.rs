use crate::reader::*;

pub fn run() {
}

fn calc(mass : i32) -> i32 {
    let f = (mass as f64 / 3.0).floor();
    f as i32 - 2
}

fn get_lines(file_path: String) -> Vec<String>{
    let reader = BufReader::open(file_path);
    let mut buffer = String::new();
    let mut vect : std::vec::Vec<std::string::String> = Vec::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        vect.push(line.unwrap().to_string());
    }
    vect
}


