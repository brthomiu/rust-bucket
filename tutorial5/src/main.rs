// Control flow

fn main() {

    // Conditional syntax examples

    let x = 2 > 1 && 1 < 2; // returns true

    let y = 1.5 >= (2 as f32) || !(1 < 2); // returns false

    if x == true {
        println!("X is true");
    } else if x == false {
        println!("X is false");
    } else {
        println!("X is not a boolean!");
    }

    println!("{} {}", x, y);

}
