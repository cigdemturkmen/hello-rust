pub fn my_var() { // pub keyword must be used for this function to be reached from other modules.

    // *** variables in rust are IMMUTABLE BY DEFAULT(they cannot be changed after they are created) ***/
    // let x = 5;
    // x = 10; // Error
    // println!("{}", x);

    // Use mut keyword to make the variable mutable
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);
}


