/*
Primitive Types
Integers: u8, i8, (both variants for 8/16/32/64/128 bits)
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays

Rust is statically typed, but can often infer the type of the variable.
*/

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Explicit type declaration
    let z: i64 = 53216531648619;

    // Maximum size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Booleans
    let is_active = true;
    let from_expression = 10 > 30;

    // A character is a unicode character
    let a1 = 'a';
    let face = '\u{1f600}'; // built in functionality

    println!("{:?}", (x, y, z, is_active, from_expression, a1, face));
}