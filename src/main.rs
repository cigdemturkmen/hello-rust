mod rust_variables;
mod rust_match;
mod rust_loops;
mod rust_functions;
mod rust_strings;

fn main() {
    rust_match::welcome();
    salute();

    // uncomment the below lines to call any functions you want from other modules

    // rust_variables::my_var();
     rust_loops::sia_chandelier();

}

fn salute() {
    // Below code works similarly to the ternary operator "?"
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);  
}

// println!() is a macro. A macro is like a function, but with an exclamation mark (!) after it.
// Macros are similar to functions (they execute things), but they do not always follow the same rules as functions.