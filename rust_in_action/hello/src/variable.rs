pub fn run() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a,b), add(c,d));

    let f = d.count_ones();
    println!("(a + b) + (c + d) = {e}");
    println!("f : {f}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
