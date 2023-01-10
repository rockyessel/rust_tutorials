#[warn(unused_variables)]

fn main() {
    //---print!()---
    //The following example prints “Rust Programming is fun, right?John is coding in Rust, cool right?” in one line.

    // print!("Rust Programming is fun, right?");
    // print!("John is coding in Rust, cool right?");

    //output Rust Programming is fun, right?John is coding in Rust, cool right? -- same line

    // ---println!()---
    //The following example prints “Rust Programming is fun, right?” on one line and “"John is coding in Rust, cool right?” on the new line.

    // println!("Rust Programming is fun, right?");
    // println!("John is coding in Rust, cool right?");

    /*
    output:
    Rust Programming is fun, right? --line
    John is coding in Rust, cool right? --another line
    */

    //---eprint!()---
    //The following example prints “Rust Programming is fun, right?” and “John is coding in Rust, cool right?” on the same line but as an error.
    // eprint!("Rust Programming is fun, right?");
    // eprint!("John is coding in Rust, cool right?");

    //output Rust Programming is fun, right?John is coding in Rust, cool right? -- same line

    // ---eprintln!()---
    //The following example prints “Rust Programming is fun, right?” as an error and appends a new line to it. Then prints “John is coding in Rust, cool right?” and appends a new line.

    // println!("Rust Programming is fun, right?");
    // println!("John is coding in Rust, cool right?");

    /*
    output as errors:
    Rust Programming is fun, right? --line
    John is coding in Rust, cool right? --another line
    */

    let a = 3;
    let mut b = 2;

    let operators = ["+", "-", "/", "%", "*", "("];

    let operators_iterator = operators.iter();

    let x = for operator in operators_iterator {
        match operator as &str {
            "+" => {
                let c = a + b;
                println!("{:?}", c)
            },
            "-" => {
                let c = b - a;
                println!("{:?}", c)
            },
            "*" => {
                let c = a * b;
                println!("{:?}", c)
            },
            "/" => {
                let c = a / b;
                println!("{:?}", c)
            },
            "%" => {
                let c = a % b;
                println!("{:?}", c)
            },
            "(" => {
                println!("invalid operator")
            },
            _ => {
                print!("None")
            }
        }
    };

    print!("{:#?}", x);



    // let ages = [27, 35, 40, 10, 19, 2, 4];

    // let mut ages_iterator = ages.iter();

    // // display the iterator
    // println!("{:?}", ages_iterator);

    // // display each element in array
    // println!("{:?}", ages_iterator.next());
    // println!("{:?}", ages_iterator.next());
    // println!("{:?}", ages_iterator.next());
    // println!("{:?}", ages_iterator.next());
    // println!("{:?}", ages_iterator.next());
    // println!("{:?}", ages_iterator.next());
    // println!("{:?}", ages_iterator.next());
    // println!("{:?}", ages_iterator.next());

    // // display the iterator
    // println!("iter_age{:?}", ages_iterator);

    // // display the array
    // println!("Age: {:?}", ages);
} // example code for using the next method.
