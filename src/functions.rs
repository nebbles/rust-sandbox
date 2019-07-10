pub fn run() {
    greeting("Hi", "John");

    let sum = add(3, 2);
    println!("{}", sum);

    // Closure - a lambda expression (anonymous function) that closes over its environment
    let add_nums = |n1: i32, n2: i32| n1+n2;
    println!("sum is: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

// This function will return an i32
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
