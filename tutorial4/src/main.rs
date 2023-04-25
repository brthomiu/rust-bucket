// Console inputs

// Import section is called "Prelude" in rust
// Packages/libraries are called "Crates"
// Crates contain modules that are specific pieces of functionality

// Imports io module from the standard library
use std::io;

fn main() {
    println!("Hello, world!");
    // Create a mutable string
    let mut input = String::new();

    // Standard input prompts user for console input
    // (&mut input) Creates a mutable reference to the input variable
    // This way changes will be applied to the original variable when the reference is passed
    // Otherwise you are just passing a new variable that is a copy of the input
    // .expect checks if the input is valid, throws error if it can't read the line
    // The types need to match, in this case input type is a string
    // If it was typed as an integer and you passed a string it would throw an error

    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You typed: {}", input);
}
