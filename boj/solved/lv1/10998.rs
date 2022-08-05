use std::{
    fs::File,
    io::{self, Read},
};

const DEBUG_MODE: u8 = 1;

fn main() {
    let mut input = String::new();
    if DEBUG_MODE != 0 {
        let mut f = File::open("./src/input.txt").unwrap();
        f.read_to_string(&mut input).unwrap();
    } else {
        io::stdin().read_to_string(&mut input).unwrap();
    }
    let mut input_iter = input.split_ascii_whitespace();

    let mut A = input_iter.next().unwrap().parse::<i32>().unwrap();
    let mut B = input_iter.next().unwrap().parse::<i32>().unwrap();
    println!("{}", A * B);
}
