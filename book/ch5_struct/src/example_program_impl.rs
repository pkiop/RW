#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("test : {:?}", rect1.area())
}
