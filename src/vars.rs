// Variables hold primitive data or reference of data
// Variables are immutable by default (value can't be changed once assigned)
// Rust is a block-scoped language

pub fn run() {
    let name = "Adithya";
    let mut age = 26;
    println!("Name is {}, I am {}", name, age);
    age = 25;
    println!("Before my birthday I was {}", age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (her_name, her_age) = ("Baz", "25");
    println!("GF is {} and she is {}", her_name, her_age);
}
