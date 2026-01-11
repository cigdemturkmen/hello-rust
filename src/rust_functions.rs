//* pub keyword must be used for this function to be reached from other modules. */

// functions with parameters
fn greet(name: &str) {
  println!("Hello, {}!", name);
}

// fuctions with return values
fn add(a: i32, b: i32) -> i32 {
  a + b // you can omit the return keyword. (no semicolon)
}