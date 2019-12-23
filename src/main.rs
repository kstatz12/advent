pub mod calc;
pub mod parser;
pub mod one;
pub mod onep2;
pub mod two;
pub mod twop2;
pub mod computer;
fn main() {
    println!("tuple of tuples: {:?}", twop2::run());
}

