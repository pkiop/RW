use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let numbers: Vec<&str> = input.split_whitespace().collect();

    let a = match numbers[0].parse::<i32>() {
        Ok(i) => i,
        Err(_e) => {
            -1
        }
    };
    let b = match numbers[1].parse::<i32>() {
        Ok(i) => i,
        Err(_e) => {
            -1
        }
    };

    println!("{}", a + b);
}
