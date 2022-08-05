use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut inputStr = String::new();
    io::stdin().read_to_string(&mut inputStr).unwrap();
    let mut tokens = inputStr.split_ascii_whitespace();

    let a = tokens.next().unwrap().parse::<i32>();
    let b = tokens.next().unwrap().parse::<i32>();

    let mut A: i32;
    let mut B: i32;
    if let Ok(a) = a {
        A = a;
    } else {
        panic!();
    }
    if let Ok(b) = b {
        B = b;
    } else {
        panic!();
    }

    println!("{}", A - B);
    // let mut output = String::new();
    // writeln!(output, "{}", tokens).unwrap();
    // print!("{}", output);

    // if let Ok(number) = number {
    //     println!("{}", number - (2541 - 1998))
    // }
}
