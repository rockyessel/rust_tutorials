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

                the variable "y" is no longer used here.
        };
     and ends here}

     So when the code gets read from top to down, and it passes the scope, from where
     the bracket is open, when the variable "y" is done been use, and goes outside the
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



//  curl 'https://gtoaixbm.api.sanity.io/v2021-06-07/data/mutate/production'
//  -H 'Authorization: Bearer '
//  -H 'Content-Type: application/json'
//  --data-binary '{"mutations":[]}'

//  curl 'https://gtoaixbm.api.sanity.io/v2021-06-07/data/mutate/production' \
//     -H 'Authorization: Bearer sk98sS8vRzMMPN6IB0Y0yrV3L0UYKWoIEKoilBd33curhKPjyI6wd6hEyiGZDVm7lLIMRCO1pTT4rOw4xxYhhf57QPNVAs0U1ep5B0C2DwGMoHPvlpxbUXvcltacaL6sugdF3iJN8d7IjaKQ9bRxaVGCuSh6kZSh0UGDvkhiGOL9xCTf1Q9X' \
//     -H 'Content-Type: application/json' \
//     --data-binary '{"mutations":[{"patch": {"id": "7ff25ddf-91d2-44ba-8630-45493d1242b5", "inc": {"viewCount": 1}, "params":{"id": "7ff25ddf-91d2-44ba-8630-45493d1242b5"}}}]}'