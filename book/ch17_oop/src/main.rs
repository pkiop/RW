mod lib;

fn main() {
    println!("Hello, world!");
    let btn = lib::Button::from(lib::Button {
        width: 32,
        height: 32,
        label: String::from("hello"),
    });
}
