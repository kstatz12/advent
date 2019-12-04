pub mod reader;
pub mod one;
fn main() {
    let x = one::run();
    println!("{}", x);
}

