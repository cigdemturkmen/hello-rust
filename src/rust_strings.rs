// There are two main types of strings in Rust:

// &str - is called "string slices", and is used for fixed text like "Hello"
// String - used when you need a string that can change

fn string_slices() {
    let greeting_slice = "Hello, world!"; // string slice - does not own memory
    let greeting_string = "Hello, world!".to_string(); // String - has its own memory
    print_string(greeting_slice);
    print_string(&greeting_string); // passing a reference(&) to avoid ownership transfer. without & it would give an error
}

fn print_string(s: &str) {
    println!("{}", s);
}

// SOME STRING FUNCTIONS AND MACROS

// Concetanation
fn concat_strings() {
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = format!("{} {} {}", s1, s2, s3);
    println!("{}", result);
}

// String Length
fn string_length() {
    let my_string = String::from("Hello, Rust!");
    let length = my_string.len();
    println!("The length of '{}' is {}.", my_string, length);
}

// push_str and push
fn push_to_string() {
    let mut my_string = String::from("Hello"); // this has to be mutable
    my_string.push_str(", world!"); // adds a string slice
    my_string.push('!'); // adds a single character
    println!("{}", my_string);
}