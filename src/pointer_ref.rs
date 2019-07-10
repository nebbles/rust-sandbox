// Reference pointers point to memory

pub fn run() {
    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("arrays: {:?}", (arr1, arr2));

    // With non-primitives, vectors...

    let vec1 = vec![1, 2, 3]; // this will be assigned to the heap

    // let vec2 = vec1; <-- this would effectively rename from vec1 to vec2
    // so instead Rust transfers the ownership of vec1's data to vec2
    // vec1 would now effectively be useless
    
    // So instead we can borrow, using the & symbol
    // We can create as many borrows as we like.
    // However, when something is being borrowed, it is in read-only mode
    // We can now only change the value once the borrows have been killed off
    // Borrows are removed once they hit the end of their scope

    let vec2 = &vec1; // this is setting vec2 to the reference of vec1

    // Since vec1 is currently being borrowed by vec2, we must borrow it again
    // for the statement below.
    println!("vecs: {:?}", (&vec1, vec2));
    // We needed to add an & to vec1 because it was being borrowed by vec2

    // We can also borrow a mutable copy with &mut but this can only be done
    // once per variable.

    // A very useful video on Rust ownership: 
    // https://www.youtube.com/watch?v=8M0QfLUDaaA
}