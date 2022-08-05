// Result<T, E> <- recoverable error
// panic! <- unrecoverable error

// when panic occur, program starts unwinding
// -> walks back up the stack, and cleans up the data from each function

/*

enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/
// fn main() {
//     println!("Hello, world!");
//     panic!("crash and burn");

//     let v = vec![1, 2, 3];

//     v[99]; // buffer overread
//            //run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// }

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};
// fn main() {
//     let f = File::open("./src/hhe.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("./src/meme.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file : {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
//     println!("f : {:?}", f);
// }

fn t() -> Result<String, io::Error> {
    let f = File::open("./src/hhe.txt")?;
    let s = String::from("hello");
    Ok(s)
}

fn main() {
    // ? -> early return
    let s = t().unwrap();
    println!("{}", s);
}
