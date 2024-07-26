use std::io;
use expression_calculator::{SuffixExpression};

fn main() {
    println!("The calculating of the expression in suffix notation");

    let mut expr_str = String::new();
    println!("Enter the expression");
    io::stdin()
        .read_line(&mut expr_str)
        .expect("Failed to read line");

    let expression = SuffixExpression::from_string(&expr_str);

    let result = expression.compute();
    println!("Result: {}", result);
}

