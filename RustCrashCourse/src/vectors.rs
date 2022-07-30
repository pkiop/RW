use std::mem;
// Vector - flexable list where elements are the same data types
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // second value is length

    println!("{:?}", numbers);
    println!("get single val : {:?}", numbers[0]);
    println!("get single val : {:?}", numbers[1]);

    // Re-assign value
    numbers[2] = 33;
    println!("{:?}", numbers);

    // Get Vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    numbers.push(5);
    numbers.push(6);
    println!("push : {:?}", numbers);
    numbers.pop();
    numbers.pop();
    println!("push!! : {:?}", numbers);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec : {:?}", numbers);
}
