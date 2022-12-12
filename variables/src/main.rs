fn main() {
    // Destructuring in Rust
    let person_info: (u8, &str, bool) = (23, "Rocky Essel", true);

    let (age, _, _) = person_info;


    // let name: &str = person_info.1;
    println!("{age}");
}
