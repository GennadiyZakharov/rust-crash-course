use rustcourse::*;


// Static string literal - a series of bytes on the stack
pub const MSG_LITERAL: &str = "Hello, world!";

pub fn demo_primitives() {
    println!("\n>>> DEMO PRIMITIVES <<<");
    println!("Primitive data types (int, float) work like in any other language.");
    println!("However this is due to one trick that we explore in the next example.");
    let a= 2; // Type is inferred
    println!("All Rust variables are immutable by default.");
    let mut b = 3; // Mo make a mutable variable we must declare it explicitly


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

    println!("\nSome simple types — like integers, floats, booleans, and characters — implement the Copy trait.");
    println!("When you assign or pass them, the value is duplicated, not moved.");
    println!("This is the reason why their behavior look sthe same as in other languages.");

    println!("\nTo demonstrate ownership, we will use a function that takes ownership of a String variable.");


    let msg = String::from(MSG_LITERAL); //

    println!("Using function with a direct link, we transfer ownership");
    print_with_own(msg);

    println!("At the end of the function scope the variable was destroyed");
    println!("We can't use the variable after the function call");
    //print_with_own(msg);
}

pub fn demo_borrowing() {
    println!("\n>>> DEMO BORROWING <<<");
    println!("To allow a function to use variable, we pass to it reference");
    println!("This process is called BORROWING");

    let msg = String::from(MSG_LITERAL);
    let msg_ref = &msg;
    print_with_borrow(msg_ref);
    println!("We still OWN the variable, so we can use the same reference many times");
    println!("We can even make references from references");
    print_with_borrow(&msg_ref);
    println!("The variablbe will be destryoed when the owner goes out of scope");
    println!("Rust ensures at compile time that the reference never outlives the variable owner");
}

pub fn demo_mutability() {
    println!("\n>>> DEMO MUTABILITY <<<");
    println!("To change a variable we must declare it as mutable");
    println!("Now we can change variable content both directly, and via functions");
    let mut mut_msg = String::from(MSG_LITERAL);
    println!("The message is: {}", mut_msg);
    // Now we can use functions that modify our structure
    mut_msg.push_str(" is mutable!");
    println!("Now the same variable holds the message: {}", mut_msg);

    println!("\nVery often we create IMMUTABLE reference to a mutable structure. This ensures that variable won't be modified dby mistake");
    let immute_ref = &mut_msg;
    println!("We pass variable to println!() by immutable reference: {}", immute_ref);

    println!("\nWe can also create MUTABLE reference to a mutable structure");
    let mut_ref = &mut mut_msg;
    println!("With this reference, we can modify the CONTENT of the variable");
    mut_ref.push_str(", is mutable via mut ref!");
    println!("Now the same variable holds the message: {}", mut_ref);
    println!("We can have simultaneously either many immutable references, or ONE mutable");
    println!("This makes Rust tread-safe, because we can't modify the same variable from different threads simultaneously");
}
//fn demo_function_mutability() {
//     println!("\n>>> DEMO FUNCTION MUTABILITY <<<");
//     println!("\nCreatign mutable variables inside functions has some tricks :)");
//  // print_and_change_string(mut_ref);
    // However, we can,t change the REFERENCE itself
    // mut_struct_ref = String::from("Hello another world"); // this won't work


    // fn print_and_change_string(msg: &mut String) {
    //     println!("Message to change: {}", msg);
    //     msg.push_str(" - was changes after printing");
    // }
    // We can create MUTABLE reference for an IMMUTABLE structure
    // let mut mut_ref= &mut_msg;
    // We can't modify the CONTENT of the structure
    // mut_ref.push_str(", modified via mut ref"); // This won't work!!!
    // But we can redirect the reference to another structure
    // let another_string = String::from("Hello another world");
    // mut_ref = &another_string;
//}

fn main() {
    println!("This example demonstrates key Rust concepts: ownership, borrowing and mutability");
    println!("Please explore the code and read comments - they are useful :)");
    demo_primitives();
    demo_ownership();
    demo_borrowing();
    demo_mutability();
}


