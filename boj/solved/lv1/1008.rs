use std::{
    env,
    fs::File,
    io::{self, Read},
};

fn main() {
    let macos: String = String::from("macos");
    let debug_mode: bool = if env::consts::OS.to_string().eq(&macos) {
        true
    } else {
        false
    };

    let mut input = String::new();
    if debug_mode {
        let mut f = File::open("./src/input.txt").unwrap();
        f.read_to_string(&mut input).unwrap();
    } else {
        io::stdin().read_to_string(&mut input).unwrap();
    }
    let mut input_iter = input.split_ascii_whitespace();

    let mut A = input_iter.next().unwrap().parse::<f64>().unwrap();
    let mut B = input_iter.next().unwrap().parse::<f64>().unwrap();
    println!("{}", A / B);
}
