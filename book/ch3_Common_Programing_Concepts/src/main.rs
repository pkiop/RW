fn main() {
    let x = 5;
    println!("The value of x is : {x}");
    const XY: u8 = 6;
    println!("The value of x is : {XY}");
    another_function(55);
}

fn another_function(x: i32) {
    println!("Another function. num is : {x}");
}
