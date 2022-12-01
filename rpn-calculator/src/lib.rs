#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for elem in inputs {
        match elem {
            CalculatorInput::Value(num) => stack.push(*num),
            CalculatorInput::Add => {
                match (stack.pop(), stack.pop()) {
                    (Some(rhs), Some(lhs)) => stack.push(lhs + rhs),
                    _ => return None
                }
            },
            CalculatorInput::Subtract => {
                match (stack.pop(), stack.pop()) {
                    (Some(rhs), Some(lhs)) => stack.push(lhs - rhs),
                    _ => return None
                }
            },
            CalculatorInput::Multiply => {
                match (stack.pop(), stack.pop()) {
                    (Some(rhs), Some(lhs)) => stack.push(lhs * rhs),
                    _ => return None
                }
            },
            CalculatorInput::Divide => {
                match (stack.pop(), stack.pop()) {
                    (Some(rhs), Some(lhs)) => stack.push(lhs / rhs),
                    _ => return None
                }
            },
        }
    }
    match stack[..] {
        [i] => Some(i),
        _     => None
    }
}
