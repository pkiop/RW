use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let numbers: Vec<&str> = input.split_whitespace().collect();
    let number = numbers[0].parse::<i32>();

    if let Ok(number) = number {
        println!("{}", number - (2541 - 1998))
    }
}
