pub fn run() {
    // Print to the console
    println!("Hello from the print.rs file");

    println!("Number is {}", 1);
    println!("{} is from {}", "John", "the UK");

    // Positional arguments - just like Python .format()
    println!("{0} is from {1}. {0} likes {2}.", "John", "the UK", "food");

    // Named arguments are also like Python
    println!("{name} is from {location}", name="John", location="the UK");

    // Can also do inline string numeric formatting
    println!("Binary {0:b} Hex {0:x} Octal {0:o}", 10);

    // Can take tuples etc.
    println!("{:?}", (12, "hello", true));
}