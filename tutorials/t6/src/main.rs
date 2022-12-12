use std::io;

fn main() {
    // Arithmetic And Type Casting //

    let a = 250u32;
    let b = 21u32;

    let c = a + b;
    println!("Addition: {}", c);

    // Subtraction
    let s = a - b;
    println!("Subtraction: {}", s);

    // Multiplication
    let d = a * b;
    println!("Multiplication: {}", d);

    // Modulus
    let m = a % b;
    println!("Modulus: {}", m);

    // Division
    let o: f32 = 290.0;
    let p: f32 = 5.0;

    let g = o / p;
    println!("Division: {}", g);

    // Type Casting

    let n = 213_000 as i64;
    let k = 32_i32;

    let v = n / (k as i64);
    println!("Type Casting: {}", v);


    // Converting a number string into an integer

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Error reading input");

    let int_input: i64 = user_input.trim().parse().unwrap();

    // trim: 
    // parse:
    //unwrap: 

    println!("The input is: {}, Integer is {}", user_input, int_input + 1000);
}
