pub fn run_print_cmd() {
    // Basic Print
    println!("PRINT FROM print.rs FILE");

    // Basic Formatting
    println!("My Name is {} and I am {} years old", "Adithya", 26);

    // Positional Argument
    println!(
        "My Name is {0} and I am {1} years old. {1} is very OLD :(",
        "Adithya", 26
    );

    // Named arguments
    println!(
        "{name} like playing {activity}. {name} works in {company}",
        name = "Adithya",
        activity = "Football",
        company = "Master's India"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 % 2 = {}", 10 % 2);
}
