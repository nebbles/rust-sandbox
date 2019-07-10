// Arrays are fixed length of elements of a single type.

use std::mem; // brings mem into the current namespace

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    let mut other_numbers: [i32; 5] = [1,2,3,4,5]; // made mutable

    other_numbers[2] = 20;

    println!("{:?}", numbers);
    println!("{:?}", other_numbers);

    // Single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length; {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

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
    let slice: &[i32] = &numbers[len-2..];
    println!("Slice 2 items from the end until the end: {:?}", slice);
}