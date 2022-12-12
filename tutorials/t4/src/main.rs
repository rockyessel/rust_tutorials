fn main() {
    // Scalar Types //
    let x: i32 = 2;
    println!("{x}");
    // Types of Integers in Rust
    //i8 -2^7 - 2^7 -1;
    //i16;
    //i32;
    //i64;
    //i128;

    //u8 0 -2^8 -1;
    //u16;
    //u32;
    //u64;
    //u128;

    // floating Point
    //f32 f64;

    let floating_point = 10.92;
    println!("{floating_point}");

    //Booleans
    let true_or_false: bool = true;
    println!("{true_or_false}");

    //Character
    let letter: char = 'e';
    println!("{letter}");

    // Scalar Types End ///

    // Compound Types //
    // Tuple is a fixed length and immutable array type.

    let tup: (i32, bool, char) = (1, true, 's');

    let tup_1: (i64, char, bool) = (23, 'f', false);

    // Destructuring in Rust
    let person_infor: (u8, &str, bool) = (23, "Rocky Essel", true);
    let (age, name , is_working) = person_infor;
    println!("{name}");

    // Accessing specific types from a tuple.
    // To access the first element, we do "tup.0" // 1
    // To access the second element, we do "tup.1" // true
    // To access the third element, we do "tup.0" // s

    println!("{}", tup.0);
    println!("{}", tup_1.2);

    // Trying to modify the tup: mut
    let mut tup: (i32, bool, char) = (1, true, 's');
    tup.0 = 1000;

    println!("{}", tup.0);


    // println!("{:#?}", tup)
    // note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print)


    //Arrays

    let mut arr:[i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("{:#?}", arr); // Displaying the array using the right formatter.
    println!("{}", arr[6]); // Accessing value for the 6th element in the array.

    arr[9] = 32;

    println!("{}", arr[9]);



    
    // Compound Types End //
}
