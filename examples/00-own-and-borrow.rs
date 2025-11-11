use rustcourse::*;


// Static string literal - a series of bytes on the stack
pub const MSG_LITERAL: &str = "Hello, world!";
pub const MSG_LITERAL_2: &str = "Hello, a new world!";

pub fn demo_primitives() {
    println!("\n>>> DEMO PRIMITIVES <<<");
    println!("Primitive data types (int, float) work like in any other language.");
    println!("However this is due to one trick that we explore in the next example.");
    let a= 2; // Type is inferred
    println!("All Rust variables are immutable by default.");
    let mut b = 3; // To make a mutable variable, we must declare it explicitly
    let c = sum2(a, b); // To function we transfer copies of the values
    println!("The sum of {} and {} is {}",a,b,c);

    println!("To change the value of a variable, we need to make it mutable.");
    // a = 5; // this won't work
    b = 7; // This works fine

    println!("However, we can shadow the previous variable and re-assign it. Even with changing type");
    let c = sum2(a, b);
    println!("The sum of {} and {} is {}",a,b,c);
}

pub fn demo_ownership() {
    println!("\n>>> DEMO OWNERSHIP <<<");
    println!("OWNERSHIP MODEL is a key concept in Rust ===");
    println!("Each value has exactly one owner (variable)");
    println!("When the owner goes out of scope, the value is dropped");

    println!("\nSome simple types — like integers, floats, booleans, and characters — implement the `Copy` trait.");
    println!("When you assign or pass them, the value is duplicated, not moved.");
    println!("This is the reason why their behavior looks the same as in other languages.");

    println!("\nTo demonstrate ownership, we will use a function that takes ownership of a String variable.");

    let msg = String::from(MSG_LITERAL); //

    println!("Using function with a direct link, we transfer ownership");
    print_with_own(msg);

    println!("At the end of the function scope the variable was destroyed");
    println!("We can't use the variable after the function call");
    //print_with_own(msg);
}

pub fn demo_borrowing() {
    println!("\n>>> DEMO REFERENCES AND BORROWING <<<");
    println!("To allow a function to use variable, we pass to it a reference");
    println!("This process is called BORROWING");
    println!("Rust uses the `&` sign to make a reference: ref = &var - reference to a variable");
    println!("To dereference, use the `*` sign: println(*ref)");
    println!("\nRust is smart about dereferencing in many situations.
    It automatically dereferences references when there is no ambiguity
    However, it could be confusing sometimes when you forgot about it");

    let msg = String::from(MSG_LITERAL);
    let msg_ref = &msg;
    print_with_borrow(msg_ref);
    println!("We still OWN the variable, so we can use the same reference many times");
    println!("\nRust can automatically dereference nested references");
    let msg_ref2 = &msg_ref;
    println!("Here Rust dereferenced &&&String (ref to ref to ref to String) -> String: {}", &msg_ref2);
    println!("The variable will be destroyed when the owner goes out of scope");
    println!("Rust ensures at compile time that the reference never outlives the variable owner");
}

pub fn demo_mutability() {
    println!("\n>>> DEMO MUTABILITY <<<");
    println!("To change a variable we must declare it as mutable");
    let mut mut_msg = String::from(MSG_LITERAL);
    println!("The message is: {}", mut_msg);
    println!("\nWhen me create a mutable variable we can:");
    println!("1. Reassign variable to a different object. The old one will be cleared");
    mut_msg = String::from(MSG_LITERAL_2);
    println!("Now the same variable holds the message: {}", mut_msg);
    println!("2. Use methods that change the variable content. For example");
    mut_msg.push_str(" is mutable!");
    println!("Now the same variable holds the message: {}", mut_msg);

    println!("\nVery often we create IMMUTABLE reference to a mutable structure. This ensures that variable won't be modified dby mistake");
    let immut_ref = &mut_msg;
    println!("We pass variable to println!() by immutable reference: {}", immut_ref);

    println!("\nWe can also create MUTABLE reference to a mutable structure");
    let mut_ref = &mut mut_msg;
    println!("With this reference, we can modify the CONTENT of the variable");
    mut_ref.push_str(", is mutable via mut ref!");
    println!("Now the same variable holds the message: {}", mut_ref);
    println!("\nWe can have simultaneously either many immutable references, or ONE mutable");
    println!("This makes Rust tread-safe, because we can't modify the same variable from different threads simultaneously");
}

fn demo_function_mutability() {
    println!("\n>>> DEMO FUNCTION MUTABILITY <<<");
    println!("Creating mutable function parameters has some tricks :)\n");
    let mut msg = String::from(MSG_LITERAL);
    println!("\n1. We can apply the mut keyword to reference: fn change_msg(msg: &mut String)");
    println!("   This allows to modify original object inside a function.");
    print_and_change(&mut msg);
    println!("Now the same variable holds the message: {}", msg);
    println!("This is a common case why we use mutable references");
    println!("Many traits use mutable &self reference");

    println!("\n2. We can apply the mut keyword to BINDING: fn change_msg(mut msg: &String)");
    println!("This allows to assign a new data to a variable, not modify the original object.");
    println!("It's a mode specific case, so we won't cover it now.")
}

fn main() {
    println!("This example demonstrates key Rust concepts: ownership, borrowing and mutability");
    println!("Please explore the code and read comments - they are useful :)");
    demo_primitives();
    demo_ownership();
    demo_borrowing();
    demo_mutability();
    demo_function_mutability();
}


