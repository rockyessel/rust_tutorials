use std::io;

//Getting user inputs

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading");

    println!("User input is : {}", input)
}
