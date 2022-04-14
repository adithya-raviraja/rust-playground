/*
Primitive Types--
Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Chars: (char)
Boolean: (bool)
Tuples
Arrays
Compiler can auto detect the type of variable during assingment. Although rust is statically typed language
 */

pub fn run() {
    // Default in i32
    let test_int = 32;
    // Default is f64
    let test_float = 12.123;

    // Add explicit type
    let y: i64 = 1234561234;

    // get_max_size
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 < 5;

    let test_char = 'a';
    let face = '\u{1F600}';

    println!(
        "{:?}",
        (test_int, test_float, y, is_active, is_greater, test_char, face)
    );
}
