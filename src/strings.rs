// Primitive str is an immuatable fixed length string in memory.
// A String is growable (allocated in heap) which is more flexible on usage.

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length - works for both str and String
    println!("Length is {}", hello.len());

    hello.push('W'); // for chars
    hello.push_str("orld!"); // for adding primitive str

    // Capacity in bytes
    println!("Capacity {}", hello.capacity());

    // Check empty
    println!("Is empty? {}", hello.is_empty());

    // Contains
    println!("Contains 'World' ? {}", hello.contains("World"));

    // .replace(before, after) replace before with after

    println!("{}", hello);

    // Looping with text
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertions
    assert_eq!(10, s.capacity()); // this should pass
    assert_eq!(3, s.len()); // this will fail
}
