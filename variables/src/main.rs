// fn main() {
//     // Destructuring in Rust
//     let person_info: (u8, &str, bool) = (23, "Rocky Essel", true);

//     let (age, _, _) = person_info;

//     // let name: &str = person_info.1;
//     println!("{age}");

//     let mut counter = 0;

//     //Loop
//     let result: i32 = loop {
//         counter += 1;

//         // breaking the loop, and putting a variable beside the "break", will make it return that variable after the break.
//         if counter == 10 {
//             break counter;
//         }
//     };

//     println!("Counter is: {result}");

//     // Classic While loop
//     let mut number: i32 = 10;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     // For each loop

//     let a: [u32; 9] = [12, 323, 43, 53, 23, 4, 342, 323, 12];

//     for number in a.iter() {
//         println!("{}", number);
//     }

//     for x in 1..100 {
//         println!("{}", x);
//     }

//     // Comments
//     // -- Line Comments

//     /*
//     Block Comments
//     */
// }

