// Loops

pub fn run() {
    let mut count = 0;

    // infinite
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            break;
        }
    }

    // While loop for fizz buzz
    println!("Starting the fizzbuzz challenge");
    count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("Count {}", count);
        }
        count += 1;
    }

    // For range
    println!("-- For range section --");
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("Count {}", x);
        }
        count += 1;
    }
}