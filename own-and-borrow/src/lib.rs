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
    // MUTABILITY AND REFERENCES
    // To change the variable and assigned data structure, we need to specify it as mutable
    let mut mut_msg = String::from(MSG_LITERAL);
    // Now we can use functions that modify our structure
    mut_msg.push_str(" is mutable!");

    // We can create simple immutable references
    let immute_ref = &mut_msg;
    print_string(&immute_ref);

    // Usually we create immutable reference to a MUTABLE structure
    let mut_struct_ref = &mut mut_msg;
    // With this reference, we can modify the CONTENT of the structure
    mut_struct_ref.push_str(", is mutable via mut ref!");
    print_and_change_string(mut_struct_ref);
    // However, we can,t change the REFERENCE itself
    // mut_struct_ref = String::from("Hello another world"); // this won't work

    // We can create MUTABLE reference for an IMMUTABLE structure
    let mut mut_ref= &mut_msg;
    print_string(mut_ref);
    // We can't modify the CONTENT of the structure
    // mut_ref.push_str(", modified via mut ref"); // This won't work!!!
    // But we can redirect the reference to another structure
    let another_string = String::from("Hello another world");
    mut_ref = &another_string;
    print_string(mut_ref);
}

fn sum2(a:i32, b:i32) -> i32 {
    a+b // implicit return in Rust
}

fn print_string_own(msg: String) {
    println!("The message form owner is: {}", msg)
}


fn print_string(msg: &String) {
    println!("Message: {}", msg)
}

fn print_and_change_string(msg: &mut String) {
    println!("Message to change: {}", msg);
    msg.push_str(" - was changes after printing");
}