#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let greeting = greet_visitor(&name);

    println!("{greeting}");
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name) // Lend the name variable to the read_line function
        .expect("Failed to read line"); // Expect the function work or terminate the program
    println!();

    name.trim().to_string()
}

fn greet_visitor(name: &str) -> String {
    let visitor_list = ["bert", "steve", "fred"];

    let mut allow_visitor_in = false;
    for visitor in &visitor_list {
        if visitor.eq_ignore_ascii_case(name) {
            allow_visitor_in = true;
            break;
        }
    }

    if allow_visitor_in {
        format!("Welcome {name}")
    } else {
        "Sorry, you are not on the list.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::greet_visitor;

    #[test]
    fn greet_bert() {
        assert_eq!(greet_visitor("Bert"), "Welcome Bert");
    }

    #[test]
    fn greet_steve() {
        assert_eq!(greet_visitor("Steve"), "Welcome Steve");
    }

    #[test]
    fn greet_fred() {
        assert_eq!(greet_visitor("Fred"), "Welcome Fred");
    }

    #[test]
    fn greet_mike() {
        assert_ne!(greet_visitor("Mike"), "Sorry, you are not on the list");
    }
}