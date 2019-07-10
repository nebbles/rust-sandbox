// Structs - Used to create custom data types
// Similar to classes and form the basis for class like design

struct Colour {
    red: u8,
    green: u8,
    blue: u8,     // trailing comma allowed
}

// Tuple struct
struct Obj (u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

// An implement
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        return Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    
    // Get full name
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    // Setter for last name - get mutable reference to self
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        return (self.first_name, self.last_name);
    }
}

pub fn run() {
    let mut c = Colour {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.blue = 50;
    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    let mut t = Obj(255, 0, 0);
    t.1 = 125;
    println!("Object: {} {} {}", t.0, t.1, t.2);

    let mut p = Person::new("John", "Smith");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Wick");
    println!("Person {}", p.full_name());
    println!("As a tuple: {:?}", p.to_tuple());
}