use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    user_defined_words: HashMap<String, Vec<String>>,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn resolve(forth: &Forth, key: &String) -> Vec<String> {
    match &forth.user_defined_words.get(key) {
        Some(v) => v
            .iter()
            .map(|k| resolve(forth, k))
            .collect::<Vec<Vec<String>>>()
            .concat(),
        None => vec![key.to_string()],
    }
}

fn parse_word_definition(forth: &mut Forth, tokens: &mut [String]) -> Result {
    let mut iter = tokens.iter();
    let variable_name = iter.next().ok_or(Error::InvalidWord)?;
    if let Some(c) = variable_name.chars().next() {
        if !(c.is_alphabetic() || matches!(c, '+' | '-' | '*' | '/')) {
            return Err(Error::InvalidWord);
        }
    } else {
        return Err(Error::InvalidWord);
    }
    for (i, token) in iter.enumerate() {
        let mut resolved = resolve(forth, token);
        forth
            .user_defined_words
            .entry(variable_name.to_string())
            .and_modify(|v| {
                if i == 0 {
                    v.clear();
                }
                v.append(&mut resolved);
            })
            .or_insert(resolved);
    }
    Ok(())
}

fn apply_arithmetic<F>(forth: &mut Forth, op: F) -> Result
where
    F: Fn(i32, i32) -> i32,
{
    match (forth.stack.pop(), forth.stack.pop()) {
        (Some(rhs), Some(lhs)) => {
            forth.stack.push(op(lhs, rhs));
            Ok(())
        }
        _ => Err(Error::StackUnderflow),
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input
            .split_whitespace()
            .map(|t| t.to_ascii_lowercase())
            .collect::<Vec<String>>();
        let mut cursor = 0;
        while cursor < tokens.len() {
            match &tokens[cursor][..] {
                num if num.parse::<i32>().is_ok() => {
                    self.stack.push(num.parse::<i32>().unwrap());
                }
                word if self.user_defined_words.get(word).is_some() => {
                    if let Some(words) = self.user_defined_words.get(word) {
                        tokens.remove(cursor);
                        for (i, w) in words.iter().enumerate() {
                            tokens.insert(cursor + i, w.to_owned());
                        }
                        continue;
                    }
                }
                "+" => apply_arithmetic(self, Add::add)?,
                "-" => apply_arithmetic(self, Sub::sub)?,
                "*" => apply_arithmetic(self, Mul::mul)?,
                "/" => match self.stack.last() {
                    Some(0) => return Err(Error::DivisionByZero),
                    Some(_) => apply_arithmetic(self, Div::div)?,
                    None => return Err(Error::StackUnderflow),
                },
                "dup" => match self.stack.last() {
                    Some(n) => self.stack.push(*n),
                    None => return Err(Error::StackUnderflow),
                },
                "drop" => {
                    if self.stack.pop().is_none() {
                        return Err(Error::StackUnderflow);
                    }
                }
                "swap" => match self.stack.len() {
                    len if len >= 2 => self.stack.swap(len - 1, len - 2),
                    _ => return Err(Error::StackUnderflow),
                },
                "over" => match self.stack.len() {
                    len if len >= 2 => self.stack.push(self.stack[len - 2]),
                    _ => return Err(Error::StackUnderflow),
                },
                ":" => {
                    match tokens[1+cursor..].iter().position(|t| t == ";") {
                        Some(index) => {
                            // Get absolute position rather than the relative to the previous slice
                            // If we don't do this the multiple definitions just find the first semicolon
                            let index = cursor + index + 1;
                            let start = cursor + 1;
                            cursor = index + 1;
                            parse_word_definition(self, &mut tokens[start..index])?;
                        }
                        None => return Err(Error::InvalidWord),
                    }
                    continue;
                }
                _ => return Err(Error::UnknownWord),
            }
            cursor += 1;
        }
        Ok(())
    }
}
