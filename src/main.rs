#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");

    let name = what_is_your_name();

    let visitor_list = ["bert", "steve", "fred"];

    let mut allow_visitor_in = false;
    for visitor in &visitor_list {
        if visitor.eq_ignore_ascii_case(&name) {
            allow_visitor_in = true;
            break;
        }
    }

    let greeting = if allow_visitor_in {
        format!("Welcome {name}")
    } else {
        "Sorry, you are not on the list.".to_string()
    };

    println!("Hello, {greeting}");
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name) // Lend the name variable to the read_line function
        .expect("Failed to read line"); // Expect the function work or terminate the program
    println!();

    name.trim().to_string()
}
