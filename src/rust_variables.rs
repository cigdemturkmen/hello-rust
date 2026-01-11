pub fn my_var() {
    // *** variables in rust are IMMUTABLE BY DEFAULT(they cannot be changed after they are created) ***/
    // let x = 5;
    // x = 10; // ERROR!!!: cannot assign twice to immutable variable
    // println!("{}", x);

    // Use mut keyword to make the variable mutable otherwise it will behave like a constant.
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);
}

// SHADOWING IN RUST
pub fn shadowing() {
    let x = 5;
    let x = 10;

    println!("x is: {}", x); // prints 10

    let spaces = "   ";
    let spaces = spaces.len(); // shadows the previous spaces variable with a new one of different type

    println!("Number of spaces: {}", spaces); // prints 3
}
