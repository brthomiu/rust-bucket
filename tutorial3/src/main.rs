// Data types

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
}
