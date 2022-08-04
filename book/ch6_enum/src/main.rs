mod coin;
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

// Option
// Null <- inventor's mistake
// No Null in rust
// but concept of null is useful. so make Option
// null <- value that currently invalid or absend for some resion

/*
enum Option<T> {
    None,
    Some(T),
}

// is std, defalut. no need to use "Option::"
*/
fn main() {
    println!("Hello, world!");

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let my_home = IpAddr::V4(String::from("127.0.0.1"));
    let my_loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap();
    println!("sum : {sum}");
    let none: Option<i32> = None;
    none.unwrap_or(32);

    coin::run();
}
