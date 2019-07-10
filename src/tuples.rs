// Tuples allow values of different types
// Maximum 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("John", "Smith", 29);

    println!("{} {} is {}", person.0, person.1, person.2);
}