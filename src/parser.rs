use super::define::TokenType;
use super::lexer;
mod plus;

pub struct express(String, TokenType);
pub struct Statements {
    code: String,
    expression: Option<express>,
}

impl Statements {
    pub fn new(src: String) -> Statements{
        Statements { code: src, expression: None }
    }
    pub fn parse(&mut self) {
        let mut lex = lexer::Lexer::new(self.code.to_string());
        let num1 = lex.next_token_is(TokenType::Num);
        let num1 = num1.1.unwrap();
        lex.next_token_is(TokenType::Plus);
        let num2 = lex.next_token_is(TokenType::Num).1.unwrap();

        self.expression = Some(Statements::plus(num1, num2));
    }
}

mod test {
    use super::Statements;
    #[test]
    pub fn test_for_plus() {
        let mut stm = Statements::new("1+1".to_string());
        stm.parse();
        println!("{:?}", stm.expression.unwrap().0);
    }
}
