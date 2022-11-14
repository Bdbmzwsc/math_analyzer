use crate::define::TokenType;
use crate::parser::express;
use crate::parser::Statements;

impl Statements {
    pub fn plus(num1: String, num2: String) -> express {
        let n1 = num1.parse::<i32>().unwrap();
        let n2 = num2.parse::<i32>().unwrap();
        let sum = n1 + n2;
        express(sum.to_string(), TokenType::Num)
    }
}
