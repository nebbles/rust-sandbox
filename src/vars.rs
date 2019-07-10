// Variables hold primitives or references to data
// Variables are IMMUTABLE BY DEFAULT
// Rust is block scoped - as expected.

pub fn run() {
    let name = "John";

    // println!(name); <-- this does not work due to the ! macro
    println!("{}", name);
    
    let mut age = 25;
    println!("The age is {}", age);
    age = 26;
    println!("The age is {}", age);

    // Constants also exist but we must declare the type
    const PIN: i32 = 301;
    println!("The pin number is {}", PIN);

    // Can assign multiple variables at once
    let (surname, id) = ("Smith", 297);
    println!("{} has id {}", surname, id);
}