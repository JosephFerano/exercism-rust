pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

struct Definition {
    resolved: bool,
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
            resolved: false,
            key: variable_name.to_owned(),
            symbols: iter.map(|s| s.to_owned()).collect::<Vec<String>>(),
        });
        Ok(())
    }

    fn resolve(&mut self, cursor: usize, word: &str, tokens: &mut Vec<String>) {
        let definition = self
            .user_definitions
            .iter()
            .rfind(|def| def.key == *word && !def.resolved)
            .unwrap();
        tokens.remove(cursor);
        for (i, w) in definition.symbols.iter().enumerate() {
            tokens.insert(cursor + i, w.to_owned());
        }
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
                word if self.user_definitions.iter().any(|def| def.key == word && !def.resolved) =>
                {
                    self.resolve(cursor, &word, &tokens);
                    // definition.resolved = true;
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
        self.user_definitions
            .iter_mut()
            .for_each(|def| def.resolved = false);
        Ok(())
    }
}
