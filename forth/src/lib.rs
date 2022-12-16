pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

struct Definition {
    key: String,
    symbols: Vec<String>,
}

#[derive(Default)]
pub struct Forth {
    user_definitions: Vec<Definition>,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn resolve(key: &str, context: &[Definition]) -> Vec<String> {
    match context.iter()
        .rposition(|def| def.key == key) {
            Some(index) => context[index].symbols
                .iter()
                .map(|sym| resolve(sym, &context[0..index]))
                .collect::<Vec<Vec<String>>>()
                .concat(),
            None => vec![key.to_owned()],
        }
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn add_definition(&mut self, tokens: &[String]) -> Result {
        let mut iter = tokens.iter();
        let variable_name = iter.next().ok_or(Error::InvalidWord)?;
        if let Some(c) = variable_name.chars().next() {
            if !(c.is_alphabetic() || matches!(c, '+' | '-' | '*' | '/')) {
                return Err(Error::InvalidWord);
            }
        } else {
            return Err(Error::InvalidWord);
        }
        self.user_definitions.push(Definition {
            key: variable_name.to_owned(),
            symbols: iter.map(|s| s.to_owned()).collect::<Vec<String>>(),
        });
        Ok(())
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
                word if self.user_definitions.iter().any(|def| def.key == word) =>
                {
                    let def_idx = self
                        .user_definitions
                        .iter()
                        .rposition(|def| def.key == *word)
                        .unwrap();
                    let defs = &self.user_definitions[def_idx];
                    let symbols = resolve(&defs.key, &self.user_definitions[..]);
                    tokens.remove(cursor);
                    for (i, w) in symbols.iter().enumerate() {
                        tokens.insert(cursor + i, w.to_owned());
                    }
                    continue;
                }
                "+" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(rhs), Some(lhs)) => self.stack.push(lhs + rhs),
                    _ => return Err(Error::StackUnderflow),
                },
                "-" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(rhs), Some(lhs)) => self.stack.push(lhs - rhs),
                    _ => return Err(Error::StackUnderflow),
                },
                "*" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(rhs), Some(lhs)) => self.stack.push(lhs * rhs),
                    _ => return Err(Error::StackUnderflow),
                },
                "/" => match (self.stack.pop(), self.stack.pop()) {
                    (Some(0), Some(_)) => return Err(Error::DivisionByZero),
                    (Some(rhs), Some(lhs)) => self.stack.push(lhs / rhs),
                    _ => return Err(Error::StackUnderflow),
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
                    cursor += 1; // Skip semicolon
                    match tokens[cursor..].iter().position(|t| t == ";") {
                        Some(index) => {
                            let result = self.add_definition(&tokens[cursor..][..index]);
                            cursor += index + 1;
                            result?
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
