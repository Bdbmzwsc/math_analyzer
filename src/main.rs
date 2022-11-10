#[warn(unused_imports)]
use math_analyzer::lexer;
use std::io;
fn main() {
    let mut src = String::new();
    io::stdin().read_line(&mut src).expect("Error at read src");
}
