mod _match;
mod variables;

fn main() {
    _match::welcome();

    println!("Hello, world!");

    test();
    variables::my_var();
}

// println!() is a macro. A macro is like a function, but with an exclamation mark (!) after it.
// Macros are similar to functions (they execute things), but they do not always follow the same rules as functions.

fn test() {
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);

    // Below code works similarly to the ternary operator "?"
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);
}
