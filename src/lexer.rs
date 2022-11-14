use super::define::TokenType;
#[derive(PartialEq, Clone)]
pub struct Token(pub TokenType, pub Option<String>, pub usize);

pub struct Lexer {
    code: String,
    next_token: Token,
    line: usize,
    ptr: usize,
}
impl Lexer {
    pub fn new(src: String) -> Lexer {
        Lexer {
            code: src,
            next_token: Token(TokenType::None, None, 0),
            line: 0,
            ptr: 0,
        }
    }
    pub fn look_ahead(&mut self) -> TokenType {
        let token = self.get_next_token();
        self.next_token = token;
        self.next_token.2 = 1;
        return self.next_token.0.clone();
    }
    pub fn get_next_token(&mut self) -> Token {
        if self.next_token.2 != 0 {
            let token = self.next_token.clone();
            self.next_token.2 = 0;
            return token;
        } else {
            return self.match_token();
        }
    }
    pub fn look_ahead_and_skip(&mut self, assert_token: TokenType) {
        let token = self.get_next_token();
        let a = token.clone();
        if token.0 != assert_token {
            self.next_token = token;
            self.next_token.2 = 1;
        }
        println!("{:?}", a.1.unwrap());
    }
    pub fn next_token_is(&mut self, assert_token: TokenType) -> Token {
        let token = self.get_next_token();
        if token.0 != assert_token {
            panic!("Except {:?},but {:?}", assert_token, token.0);
        }
        token
    }

    pub fn match_token(&mut self) -> Token {
        if self.ptr == self.code.len() {
            return Token(TokenType::EOF, Some("".to_string()), self.line);
        }
        match &self.code[self.ptr..].chars().next().unwrap() {
            '(' => {
                self.skip_src(1);
                return Token(TokenType::LeftPrt, Some("(".to_string()), self.line);
            }
            ')' => {
                self.skip_src(1);
                return Token(TokenType::RightPrt, Some(")".to_string()), self.line);
            }

            '+' => {
                self.skip_src(1);
                return Token(TokenType::Plus, Some("+".to_string()), self.line);
            }
            '-' => {
                self.skip_src(1);
                return Token(TokenType::Minus, Some("-".to_string()), self.line);
            }
            '*' => {
                self.skip_src(1);
                return Token(TokenType::Multi, Some("*".to_string()), self.line);
            }
            '/' => {
                self.skip_src(1);
                return Token(TokenType::Divison, Some("/".to_string()), self.line);
            }
            '1'..='9' => {
                let token = self.scan_num();
                return token;
            }
            &_ => panic!("invaild token"),
        }
    }
    pub fn skip_src(&mut self, n: usize) {
        self.ptr += n;
    }
    pub fn next_code_is(&mut self, code: &String) -> bool {
        let mut it = self.code.chars().skip(self.ptr);
        for i in code.chars() {
            if let Some(ch) = it.next() {
                if i != ch {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    pub fn scan_num(&mut self) -> Token {
        let mut it = self.code.chars().skip(self.ptr);
        let mut str = String::new();
        let mut t = it.next();
        if let Some('1'..='9') = t {
            str.push(t.unwrap());
            self.ptr += 1;
            t = it.next();
            while let Some('0'..='9') = t {
                str.push(t.unwrap());
                t = it.next();
                self.ptr += 1;
            }
        } else {
            panic!("Isn't a num")
        }
        Token(TokenType::Num, Some(str), self.line)
    }
}

#[allow(unused)]
pub mod test {
    use super::Lexer;
    use crate::define::TokenType;

    #[test]

    pub fn test_for_match_token() {
        let s = String::from("123)");
        let mut l = Lexer::new(s);
        let t = l.get_next_token();
        let test = t.1;
        println!("{:?}", test.unwrap());
    }
    #[test]
    pub fn test_for_look_ahead() {
        let s = String::from("(123)");
        let mut l = Lexer::new(s);
        l.look_ahead();
        let token = l.look_ahead();
        println!("{:?}", l.next_token.2);
        println!("{:?}", token);
    }
    #[test]
    pub fn test_for_next_token_is() {
        let s = String::from("(123)");
        let mut l = Lexer::new(s);
        l.next_token_is(TokenType::LeftPrt);
        println!("{:?}", l.look_ahead())
    }
    #[test]
    pub fn test_for_lookahead_and_skip() {
        let s = String::from("(11+11)");
        let mut l = Lexer::new(s);
        l.look_ahead_and_skip(TokenType::LeftPrt);
        l.look_ahead_and_skip(TokenType::Num);
        l.look_ahead_and_skip(TokenType::Num);
    }
    #[test]
    pub fn test_for_EOF(){
        let s=String::from("1+1");
        let mut l = Lexer::new(s);
        l.next_token_is(TokenType::Num);
    }
}
