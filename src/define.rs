#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType{

    RightPrt,
    LeftPrt,
    Plus,
    Num,
    Minus,
    Multi,
    Divison,
    None,
    EOF
}