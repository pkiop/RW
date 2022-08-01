use std::io;

// fn main() {
//     println!("Guess the number!");
//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess).expect("Fail to read line");

//     println!("You guessed: {}", guess);

// }

fn main() {
    let mut a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    let mut normal = a;
    println!("{:?}", slice);
    normal[2] = 2;
    println!("{:?}", slice);
}
