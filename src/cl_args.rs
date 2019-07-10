// To gather command line arguments 

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    println!("args are {:?}", args);
    println!("Your argument: {:?}", command);

    // args.0 will be the target executable, 
    // then your own args begin from args.1

    let name = "John";
    if command == "hello" {
        println!("Hi {}", name);
    }
}