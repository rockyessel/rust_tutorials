fn main() {
    println!("Hello, world!");

    /*
    ----Ownership Rules----

    There are three basic for every rust developer has to follow in terms of ownership.
    1. Each Rust value has a variable called its "Owner"
    let x = 5; So the variable "x" is the owner of the value "5"
    Meaning the variable "x" owns the value "5"

    2. Each value, can only have one owner at a time.
    Meaning the value "5" can be owned by any variable at a time, so
    therefore, the vaule,"5", cannot be owned by two or more owners.

    3. When the owner of a value, goes out of scope, the vaule will be  dropped.
     { the scope start here
        let y:i8 = 123;

        loop {
            y += 1;

            if y >= 200 {
                return y - 100;
            }

            println!("{}", y);z

                the varaible "y" is no longer used here.
        };
     and ends here}

     So when the code gets read from top to down, and it passes the scope, from where
     the bracket is open, when the varaible "y" is done been use, and goes outside the
     scope, from where the bracket closes, then the value has been dropped.

     -------------------------------------------------------------------------------------

     When the variable goes outside the scope, the Rust ownership feature will allow for the memory to be freed.
    */

    let x = 5;


    /*
    
    Clone And Copy in Rust



    */

    let a = String::from("Rocky Essel");
    let b = a;
    println!("a: {}", a);
    println!("b: {}", b);
}
