fn main() {
    // Rust is a statically and strongly typed language
    // Types can be declared explicitly or implicitly
    let x = 4;

    println!("x is: {x}");

    // Variables are immutable by default in rust, to make them mutable use "mut"
    let mut y: u8 = 5;

    y = 6;

    println!("y is: {y}");

    {
        // Local scope in curly brackets
        let x = x - 1;
        println!("in the curly brackets, x is: {x}");
    }

    // It is possible to override a variable with a new variable if it is not mutable

    let x = x + 1;

    println!("x redeclared is: {x}");

    // Variables can be redefined as any type

    let x = "Stringy";

    println!("x redeclared as a string is: {x}");

    // Constants are NOT the same as variables
    // Should be declared in allcaps, must have a type and a value
    // Cannot be redeclared or modified

    const SECONDS_IN_MINUTE: i32 = 60;

    println!("constant is: {SECONDS_IN_MINUTE}");
}
