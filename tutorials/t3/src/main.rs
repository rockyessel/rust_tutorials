fn main() {
    
    let x = 4;
    println!("x is: {}", x);

    {
        let x = 340 + x;
        println!("x is: {}", x);

        {
            let x = 2300 + x;
            println!("x is: {}", x);
        }
    }

    let x = x + 10;
    println!("x is: {}", x);


    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}
