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

// Store the name and the greeting
// Store the visitor’s age, and forbid them from drinking if they’re under 21.
#[derive(Debug, Clone)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_string(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greeting(&self) -> String {
        match &self.action {
            VisitorAction::Accept => format!("Welcome to the tree house {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                let welcome = format!("Welcome to the treehouse, {}\n{}", self.name, note);
                if self.age < 21 {
                    format!("{}\nDo not server alcohol to {}.", welcome, self.name)
                } else {
                    welcome
                }
            }
            VisitorAction::Probation => format!("{} is now a probationary member.", self.name),
            VisitorAction::Refuse => format!("Do not allow {} in!", self.name),
        }
    }
}

// Store an action associated with a visitor: admit them, admit them with a
// note, refuse entry, or mark them as probationary treehouse members
#[derive(Debug, Clone)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
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
        Visitor::new(
            "Bert",
            "Hello Bert, enjoy your treehouse.",
            VisitorAction::Accept,
            45,
        ),
        Visitor::new(
            "Steve",
            "Hi Steve. Your milk is in the fridge.",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("Fred", "Wow, who invited Fred?", VisitorAction::Refuse, 30),
    ]
}

fn greet_visitor(visitor_list: &mut Vec<Visitor>, name: &str) -> Option<String> {
    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name.eq_ignore_ascii_case(name));

    #[allow(clippy::option_if_let_else)]
    match known_visitor {
        Some(visitor) => Some(visitor.greeting()),
        None => {
            if name.is_empty() {
                None
            } else {
                let visitor = Visitor::new(
                    name,
                    format!("Hello {name}").as_str(),
                    VisitorAction::Probation,
                    0,
                );
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
        greet_visitor_test("Bert", "Welcome to the tree house Bert");
    }

    #[test]
    fn greet_steve() {
        greet_visitor_test(
            "Steve",
            "Welcome to the treehouse, Steve\n\
            Lactose-free milk is in the fridge\n\
            Do not server alcohol to Steve.",
        );
    }

    #[test]
    fn greet_fred() {
        greet_visitor_test("Fred", "Do not allow Fred in!");
    }

    #[test]
    fn greet_mike() {
        greet_visitor_test("Mike", "Mike is not on the visitor list.");
    }
}
