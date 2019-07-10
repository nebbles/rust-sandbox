// Vectors are resizeable arrays

use std::mem; // brings mem into the current namespace

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;
    println!("{:?}", numbers);

    // Add onto vectors
    numbers.push(8);
    numbers.push(9);
    numbers.pop();
    println!("Updated: {:?}", numbers);

    // Single value
    println!("Single value: {}", numbers[0]);

    // Get Vector length
    println!("Vector length; {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice first two: {:?}", slice);

    let slice: &[i32] = &numbers[1..3];
    println!("Slice index 1&2: {:?}", slice);

    let slice: &[i32] = &numbers[..2];
    println!("Slice everything up to index 2: {:?}", slice);

    let slice: &[i32] = &numbers[2..];
    println!("Slice from index 2 onwards: {:?}", slice);

    let len = numbers.len();
    let slice: &[i32] = &numbers[len - 2..];
    println!("Slice 2 items from the end until the end: {:?}", slice);

    // Looping through the vector values
    for x in numbers.iter() {
        println!("Number {}", x);
    }

    // Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);
}