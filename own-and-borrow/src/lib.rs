use String;
// this is a string literal
pub const MSG_LITERAL: &str = "Hello, world!";

pub fn demo_primitives() {
    // Primitive data types are created on stack
    // They works like in any other language
    let a= 2;
    let mut b = 3;
    let c = sum2(a,b); // To function we transfer copies of the values
    // We can use original a and b values after function
    println!("The sum of {} and {} is {}",a,b,c);

    // We can change value only for mutable variables
    // a = 5; // this won't work
    b = 7; // This works fine

    // However, we can shadow previous variable and re-assign it.
    // We can change variable type at this point and re-use name
    let c = sum2(a,b);
    println!("The sum of {} and {} is {}",a,b,c);
}

pub fn demo_ownership() {
    // OWNERSHIP FOR DATA ON THE HEAP
    // TRANSFERRING OWNERSHIP
    // Dynamic data values are allocated on heap
    // This variable is String type. But this is the link to
    // dynamic String object
    let msg = String::from(MSG_LITERAL);

    // Using function with a direct link, we transfer ownership
    print_string_own(msg);

    // At the end of the function scope the variable was destroyed
    // The second usage of the variable is not possible
    // print_String_own(msg);
}

pub fn demo_references() {
    // USING REFERENCES
    // Making second string variable
    let msg = String::from(MSG_LITERAL);
    // Here we transfer to the function a _reference_ to the String structure
    let msg_ref = &msg;
    print_string(msg_ref);
    // We still OWN the variable, so we can use the same reference many times
    // We can even make references form references
    print_string(&msg_ref);
    // here we reach the end of the scope - the variable will be destroyed here
}

pub fn demo_mutability() {
    // MUTABILITY FOR REFERENCES
    // To change the variable, we need to specify it as mutable
    let mut mut_msg = String::from(MSG_LITERAL);
    // Now we can use functions that modify our structure
    mut_msg.push_str(" is mutable!");
    print_string(&mut_msg);

    // For mutable variable we can create mutable reference
    let mut_ref= &mut mut_msg;
    // Now we can pass reference to the functions that change data structure
    print_and_change_string(mut_ref);
    // Any function accepting immutable references, can work with mutable
    print_string(mut_ref);

    // We can create a new reference - we'll see that the original value has benn changes
    print_string(&mut_msg);
}

fn sum2(a:i32, b:i32) -> i32 {
    a+b // implicit return in Rust
}

fn print_string_own(msg: String) {
    println!("The message form owner is: {}", msg)
}


fn print_string(msg: &String) {
    println!("The message is: {}", msg)
}

fn print_and_change_string(msg: &mut String) {
    println!("The message to change is: {}", msg);
    msg.push_str(" - was changes after printing");
}