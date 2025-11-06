// library file that holds some shared functions, useful for demo

pub fn sum2(a:i32, b:i32) -> i32 {
    a+b // implicit return in Rust
}

pub fn print_with_own(msg: String) {
    println!("The message from owner is: {}", msg)
}

pub fn print_with_borrow(msg: &String) {
    println!("Message: {}", msg)
}