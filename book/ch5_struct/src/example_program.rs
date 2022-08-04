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
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels (using tuples)",
        area_tuple(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels (using struct )",
        area_struct(&rect2)
    );

    println!("print struct {:#?}", rect2);
    dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
