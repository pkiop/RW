mod hash;
fn main() {
    let mut my_hello = String::from("");
    my_hello.push_str("hello");
    println!("{}", my_hello);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!, ");

    // + operator signature
    // fn add(self, s: &str) -> String {
    // &string -> &str clone occur
    // self : must have ownership
    // let s1 = s1 + &s2;
    let s3 = s1 + &s2;
    // String + String ❌
    // &str + String ✅
    // println!("s1 : {}", s1);
    println!("s2 : {}", s2);
    // println!("s3 : {}", s1);
    println!("s3 : {}", s3);

    // s3[0] // error. rust string doesn't provide indexing
    // Why??

    // 1.
    // String : wrapper over a Vec<u8>
    let hello = String::from("Hola"); // len is 4

    let hello = String::from("Здравствуйте"); // len is 12? actually 24
                                              // UTF-8, index not always correlate to a valid Unicode scalar value

    //2
    // and next resion. it doesn't always take time (O(1))
    // so can't guarantee performance
    // because it would search this charactor is valid

    // alternative. use string slice!

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s : {}", s);

    // let s = &hello[0..1]; // error ->  index 1 is not a char boundary panic!
    // println!("s : {}", s);

    // best way to get char
    // -> .chars()

    let s = "hello_world";
    for c in s.chars() {
        println!("{}", c);
    }

    hash::run();
}
