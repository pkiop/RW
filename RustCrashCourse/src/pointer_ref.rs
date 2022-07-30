// References Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let mut arr1 = [1, 2, 3];
    let mut arr2 = arr1;

    // With non-primitives, if you assign another variable to a piece of data,
    // the first variable will no longer hold that value.
    // You'll need to use a reference(&) to point to the resource
    println!("Values: {:?}", (arr1, arr2));

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    // (&vec2).push(3); // error why?

    println!("Values: {:?}", (&vec1, vec2));
}
