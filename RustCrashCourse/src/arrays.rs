use std::mem;
// Array - fixed list where elements are the same data types
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // second value is length

    println!("{:?}", numbers);
    println!("get single val : {:?}", numbers[0]);
    println!("get single val : {:?}", numbers[1]);

    // Re-assign value
    numbers[2] = 33;
    println!("{:?}", numbers);

    // Get Array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
