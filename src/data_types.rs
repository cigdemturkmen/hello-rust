//** DATA TYPES IN RUST **/
fn data_types() {
    let my_num = 5; // integer
    let my_double = 5.99; // float
    let my_letter = 'D'; // character
    let my_bool = true; // boolean
    let my_text = "Hello"; // string

    // types are indicared explicitly below
    let my_num: i32 = 5; // integer
    let my_double: f64 = 5.99; // float
    let my_letter: char = 'D'; // character
    let my_bool: bool = true; // boolean
    let my_text: &str = "Hello"; // string
}

//** CONSTANTS IN RUST **/
fn constants() {
    // You MUST write the type
    const BIRTHYEAR: i32 = 1980; // Ok
    //const BIRTHYEAR = 1980; // Error: missing type
}

