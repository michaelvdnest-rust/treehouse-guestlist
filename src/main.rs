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

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_string(),
            greeting: greeting.to_string(),
        }
    }
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
    let visitor_list = [
        Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("Steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("Fred", "Wow, who invited Fred?"),
    ];

    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name.eq_ignore_ascii_case(name));

    known_visitor.map_or(
        "You are not on the visitor list. Please leave.".to_string(), 
        |visitor| visitor.greeting.clone())
}

#[cfg(test)]
mod tests {
    use crate::greet_visitor;

    #[test]
    fn greet_bert() {
        assert_eq!(greet_visitor("Bert"), "Hello Bert, enjoy your treehouse.");
    }

    #[test]
    fn greet_steve() {
        assert_eq!(greet_visitor("Steve"), "Hi Steve. Your milk is in the fridge.");
    }

    #[test]
    fn greet_fred() {
        assert_eq!(greet_visitor("Fred"), "Wow, who invited Fred?");
    }

    #[test]
    fn greet_mike() {
        debug_assert_ne!(greet_visitor("Mike"), "You are not on the visitor list. Please leave!".to_string());
    }
}
