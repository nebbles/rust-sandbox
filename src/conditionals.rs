// Conditionals

pub fn run() {
    let age = 21; // type was inferred in this case
    let check_id: bool = false;
    let knows_person = true; // inferred type

    // if else
    if age >= 21 && check_id || knows_person {
        println!("Passed the age threshold");
    } else if age < 21 && check_id {
        println!("You have failed the age test");
    } else {
        println!("Please submit your ID");
    }

    // Shorthand if
    let is_old_enough = if age >= 21 {"yes"} else {"no"};
    let pass = age >= 21;
    println!("Passed: {}", is_old_enough);
    println!("Passed: {}", pass);
}