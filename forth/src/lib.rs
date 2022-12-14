use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    user_defined_words: HashMap<String, String>,
    stack: Vec::<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}


impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::<Value>::new(), user_defined_words: HashMap::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace();
        while let Some(token) = tokens.next() {
            match &token.to_ascii_lowercase()[..] {
                num if num.parse::<i32>().is_ok() => {
                    self.stack.push(num.parse::<i32>().unwrap());
                }
                "+" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(rhs), Some(lhs)) => self.stack.push(lhs + rhs),
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                "-" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(rhs), Some(lhs)) => self.stack.push(lhs - rhs),
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                "*" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(rhs), Some(lhs)) => self.stack.push(lhs * rhs),
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                "/" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(0), Some(_)) => return Err(Error::DivisionByZero),
                        (Some(rhs), Some(lhs)) => self.stack.push(lhs / rhs),
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                "dup" =>
                    match self.stack.last() {
                        Some(n) => self.stack.push(*n),
                        None => return Err(Error::StackUnderflow),
                    }
                "drop" =>
                    if self.stack.pop().is_none() {
                        return Err(Error::StackUnderflow)
                    }
                "swap" => {
                    let len = self.stack.len();
                    if len >= 2 {
                        self.stack.swap(len - 1, len - 2);
                    } else {
                        return Err(Error::StackUnderflow)
                    }
                }
                "over" => {
                    let len = self.stack.len();
                    if len >= 2 {
                        self.stack.push(self.stack[len - 2]);
                    } else {
                        return Err(Error::StackUnderflow)
                    }
                }
                ":" => {
                    if let Some(word_name) = tokens.next() {
                        if let Some(c) = word_name.chars().nth(0) {
                            if !c.is_alphabetic() {
                                return Err(Error::InvalidWord)
                            }
                        }
                        let word_def = Vec::<&str>::new();
                        while let Some(token) = tokens.next() {
                            // We have to figure out how to report an error
                            // if we don't find a semi colon
                            match &token.to_ascii_lowercase()[..] {
                                ";" => break,
                                _ => (),
                            }
                        }
                    } else {
                        return Err(Error::InvalidWord)
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}
