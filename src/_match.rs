use chrono::{Local, Datelike}; // chrono is a crate. a library crate.

fn match_1() {
  let day = 4;

  match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }
}

fn match_2() {
  let day = 6;

  match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  }
}

fn match_3() {
  let day = 4;

  let result = match day {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    4 => "Thursday",
    5 => "Friday",
    6 => "Saturday",
    7 => "Sunday",
    _ => "Invalid day.",
  };

  println!("{}", result);
}

pub fn welcome() {

let weekday_number = Local::now().weekday().number_from_monday();
   
  let result = match weekday_number {
    1 => "MONDAY",
    2 => "TUESDAY",
    3 => "WEDNESDAY",
    4 => "THURSDAY",
    5 => "FRIDAY",
    6 => "SATURDAY",
    7 => "SUNDAY",
    _ => "Invalid day.",
  };


  println!("WELCOME {}'S LESSON CIGDEM!! :)", result);
}