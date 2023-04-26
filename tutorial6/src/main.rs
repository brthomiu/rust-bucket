// Functions

// In Rust it is conventional to use snake_case instead of camelCase

fn main() {
    test_funct(2, 3);

    let number = {
        let x = 3;
        x + 1 // Omitting the semicolon on the last line of a function will cause it to return the value from the expression when the function is called
        // Explicit returns with a semicolon (IE: return x + 1;) are also allowed
    };

    println!("Number = {}", number);

    println!("Sum = {}", add_numbers(5, 7));
}

// The order of functions doesn't matter

// Test function that takes x and y parameters
// Typing parameters is mandatory

fn test_funct(x: i32, y: i32) {
    println!("Sum: {}", x + y);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

// Statements VS Expressions

// Functions can return an expression, they cannot return a statement.

// Statements do not return anything/have a value (ie: 'let x = 20' declares a value for x, but doesn't return anything)

// Expressions are anything that does return a value, (function returns, macro returns, primitive values, etc...)

// A function that doesn't have a return will give empty parentheses when called