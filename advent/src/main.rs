pub mod calc;
pub mod parser;
pub mod one;
pub mod onep2;
fn main() {
    let x = one::run();
    println!("{}", x);
    let x2 = onep2::run();
    println!("{}", x2);
}

