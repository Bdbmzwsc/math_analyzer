#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    RightPrt,
    LeftPrt,
    Plus,
    Num,
    Minus,
    Multi,
    Divison,
    None,
    EOF,
}
