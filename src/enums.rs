// Enums are types which have a few definite values (like constant structs)

enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action based on m
    // match is similar to a traditional switch
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run() {
    let move1 = Movement::Left;
    let move2 = Movement::Up;
    let move3 = Movement::Right;
    let move4 = Movement::Down;

    move_avatar(move1);
    move_avatar(move2);
    move_avatar(move3);
    move_avatar(move4);
}