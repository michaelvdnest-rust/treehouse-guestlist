#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let mut name = String::new();

    stdin()
        .read_line(&mut name)             // Lend the name variable to the read_line function
        .expect("Failed to read line");   // Expect the function work or terminate the program 
    println!();

    println!("Hello, {name}");
}