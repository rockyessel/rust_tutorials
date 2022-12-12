fn main() {
    // Conditions And Control Flow (if/else if/else); //

    // -- The Six Compare Operators --
    // <
    // >
    // <=
    // >=
    // !=
    // ==

    let cond = 10 < 30;
    println!("{}", cond);

    // Comparing objects with different types
    let cond_1 = (2 as f32) > 1.2; // This is known as type casting in Rust
    println!("{cond_1}");

    // -- Compound Conditions --
    // These are the three logical operators in Rust.
    // &&: This return true if both the side are true, else false.
    // ||: This return true if one side is true, and return false if both sides is false.
    // !: This changes the boolean condition, so if it si true


    let checking_and_operator: bool = true && cond;
    println!("Using the && operator, so the value is: {checking_and_operator}");

    let checking_or_operator: bool = cond || checking_and_operator;
    println!("Using the || operator, so the value is: {checking_or_operator}");

    let checking_not_operator: bool = !checking_or_operator;
    println!("Using the ! operator, so the value is: {checking_not_operator}");

    // -- Control Flow --

    let control_flow: &str = "bread";

    if control_flow != "break" {
        println!("This is true.")
    } else if control_flow == "bread" {
        println!("Dmn!")
    }else{
        println!("This is false.")
    }



}
