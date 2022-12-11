pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    Colon,
    Semicolon,
    Dup,
    Drop,
    Swap,
    Over,
    Identifier,
}

pub enum Primitive {
    Num of i16
}

pub struct Token {
    token_type: TokenType
    lexeme: String
}

impl Forth {
    pub fn new() -> Forth {
        unimplemented!()
    }

    pub fn stack(&self) -> &[Value] {
        unimplemented!()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        unimplemented!("result of evaluating '{}'", input)
    }
}
