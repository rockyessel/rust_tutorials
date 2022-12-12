use std::io;

fn main() {
    // Taking user input.
    let mut user_input_x = String::new();
    let mut user_input_y = String::new();

    // Making input editable.
    io::stdin()
        .read_line(&mut user_input_x)
        .expect("Error reading input");
    io::stdin()
        .read_line(&mut user_input_y)
        .expect("Error reading input");

    // Converting &str into int.
    let user_inpur_integer_x: u64 = user_input_x.trim().parse().unwrap();
    let user_inpur_integer_y: u64 = user_input_y.trim().parse().unwrap();

    let result:u64  = add_numbers_with_limit(user_inpur_integer_x, user_inpur_integer_y);

    println!("The total result is:{}", result);
}

fn add_numbers_with_limit(user_x_input: u64, user_y_input: u64) -> u64 {
    let added_numbers = user_x_input + user_y_input;

    if added_numbers > 100 {
        return added_numbers - 50;
    } else if added_numbers == 50 {
        return added_numbers - 25;
    } else {
        return added_numbers -59 ;
    }

    // return added_numbers;
}
