#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use std::io::stdin;

fn main() {
    let mut visitor_list: Vec<Visitor> = visitor_list();
    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();

        match greet_visitor(&mut visitor_list, &name) {
            Some(greeting) => println!("{greeting}"),
            None => break,
        }
    }

    println!("The final list of visitors");
    println!("{visitor_list:#?}");
}

#[derive(Debug, Clone)]
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

fn visitor_list() -> Vec<Visitor> {
    vec![
        Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("Steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("Fred", "Wow, who invited Fred?"),
    ]
}

fn greet_visitor(visitor_list: &mut Vec<Visitor>, name: &str) -> Option<String> {
    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name.eq_ignore_ascii_case(name));

    #[allow(clippy::option_if_let_else)]
    match known_visitor {
        Some(visitor) => Some(visitor.greeting.clone()),
        None => {
            if name.is_empty() {
                None
            } else {
                let visitor = Visitor::new(name, format!("Hello {name}").as_str());
                visitor_list.push(visitor);
                Some(format!("{name} is not on the visitor list."))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{greet_visitor, visitor_list};

    fn greet_visitor_test(name: &str, greeting: &str) {
        let mut visitors = visitor_list();
        for visitor in visitors.clone() {
            if visitor.name == name {
                assert_eq!(greet_visitor(&mut visitors, name).unwrap(), greeting);
            }
        }
    }

    #[test]
    fn greet_bert() {
        greet_visitor_test("Bert", "Hello Bert, enjoy your treehouse.");
    }

    #[test]
    fn greet_steve() {
        greet_visitor_test("Steve", "Hi Steve. Your milk is in the fridge.");
    }

    #[test]
    fn greet_fred() {
        greet_visitor_test("Fred", "Wow, who invited Fred?");
    }

    #[test]
    fn greet_mike() {
        greet_visitor_test("Mike", "Mike iw not on the visitor list.");
    }
}
