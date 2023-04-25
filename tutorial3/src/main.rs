// Data types

use std::io;

fn main() {
    // Integer

    // Integer types can be specified from 8-128bit and signed/unsigned as such
    // Variable names can be anything, in this example they correspond to the type
    // Numbers infer to i32 by default

    let _unsigned_8bit: u8 = 1;

    let _signed_8bit: i8 = 1;

    let _unsigned_16bit: u16 = 1;

    let _signed_16bit: i16 = 1;

    // etc.. up to 128 bits

    // Float

    // 32bit (single precision) and 64bit (double precision)
    // floats infer to f64 by default

    let _float32: f32 = 1.2;

    let _float64: f64 = 1.2;

    // Boolean

    // Yep, it's booleans

    let _boolean: bool = false;

    let _boolean: bool = true;

    // Char

    // Single characters (in single quotes)

    let _character: char = 'a';

    // Tuple

    // Tuple types will be inferred as what the tuples contain
    // They can also be explicitly typed (like here)

    let _tup: (i32, bool, char) = (1, true, 's');

    let _tup2: (i8, bool, bool) = (1, true, false);

    // Tuples are accessed with a . notation

    println!("{} - {}", _tup.1, _tup2.0);

    // Tuples are immutable by default, but can be made mutable with mut
    // The type of the tuple cannot be modified (that includes adding to the tuple)

    let mut _tup3: (i16, bool) = (1, true);

    // Array

    // Arrays must be explicitly declared
    // Arrays are immutable by default, but can be made mutable with mut
    // The type of the array cannot be modified (that includes adding to the tuple)

    let _arr = [1, 2, 3, 4, 5];

    // Array types can also be explicitly declared
    // Multiple items in an array can be declared with the length after the type in the type declaration
    // IE: this is an array of 5 i8s

    let _arr2: [i8; 5] = [1, 2, 3, 4, 5];



// Additional notes about arithmetic operations with types

// Different integer data types cannot be combined without specifying an implementation

// Arithmetic operations will always return the same type as the inputs

// IE: (a: i32 = 15) / (b: i32 = 10) = 1
// A float cannot be returned from an integer operation so the decimals are dropped
// This applies to all arithmetic operations (+, -, *, /, %)

// Integer types can also be declared by appending the type at the end of the number
// Underscores are ignored and can be used to organize integers

let _int = 5_000_000_i64;

let _int2 = 5_000_000_i32;

// Use 'as' to cast integers to a different type

let _int3 = _int + _int2 as i64;

println!("{_int3}, {_int}");

// It is not a good practice to cast integers down to smaller types because you can cause a memory overflow
// Overflows will wrap and not necessarily throw an error, can break things in a non-obvious way

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Could not read line");


// Trim is a string method that removes the console input character (extra invisible character added by console input)
// Parse will parse the string into an integer
// Unwrap casts the integer to the specified type
let int_input: i64 = input.trim().parse().unwrap();

println!("{int_input}");

}