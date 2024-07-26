// Calculation expressions in suffix notation
use Vec;

pub struct SuffixExpression{
    expression: Vec<Operation>
}

enum Operation {
    Value(f64),
    Action(BinaryAction)
}
enum BinaryAction {
    Add,
    Sub,
    Multiply,
    Divide,
}

struct Stack {
    stack: Vec<f64>
}

impl BinaryAction {
    fn from_char(c: char) -> BinaryAction {
        match c {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            _ => panic!("Unsupported binary operation")
        }
    }

    fn apply(&self, stack: &mut Stack) {
        let b = stack.stack.pop().unwrap();
        let a = stack.stack.pop().unwrap();
        match self {
            Self::Add => stack.stack.push(a + b),
            Self::Sub => stack.stack.push(a - b),
            Self::Multiply => stack.stack.push(a * b),
            Self::Divide => stack.stack.push(a / b)
        }
    }

}

impl Operation {
    pub fn from_string(string: &str) -> Operation {
        let char_op = string.chars().next().expect("string is empty");
        match char_op {
            '+' | '-' | '*' | '/' => {
                Operation::Action(BinaryAction::from_char(char_op))
            },
            _ => {
                Operation::Value(string.trim().parse().expect("Illegal operation"))
            }
        }
    }
}

impl SuffixExpression {
    pub fn from_string(exp_str: &str) -> SuffixExpression {
        let mut expression: Vec<Operation> = Vec::new();
        for word in exp_str.split_whitespace() {
            expression.push(Operation::from_string(word));
        }
        SuffixExpression{expression: expression}
    }
    pub fn compute(&self) -> f64 {
        let mut stack= Stack{ stack: Vec::new()};
        for operation in self.expression.iter() {
            match operation {
                Operation::Action(action) => {
                    action.apply(&mut stack);
                }
                Operation::Value(value) => stack.stack.push(*value)
            }
        }
        stack.stack.pop().unwrap()
    }
}



