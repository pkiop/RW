// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure
// - Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello"; // Primitive str
    let mut hello2 = String::from("Hello "); // Primitive str

    println!("{}", hello);
    println!("Length: {}", hello.len());

    println!("Capacity: {}", hello2.capacity());
    println!("{}", hello2);
    println!("Length: {}", hello2.len());
    hello2.push('W');
    hello2.push_str("orld");
    println!("Length: {}", hello2.len());
    println!("Capacity: {}", hello2.capacity());
    println!("Is Empty: : {}", hello2.is_empty());
    println!("Contains: 'World': {}", hello2.contains("World"));
    println!("Replace: {}", hello2.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", hello2);
    println!("{}", hello2);
}
