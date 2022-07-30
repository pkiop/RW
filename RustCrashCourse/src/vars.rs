pub fn run() {
    let a: u8 = 0;
    println!("{}", a);
    println!("Max i32: {}", std::u32::MAX);

    let is_active = true;
    let is_grater = 10 == 5;

    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (a, is_active));
    println!("{:?}", (a, is_grater));
    println!("{:?}", (a1, face))
}
