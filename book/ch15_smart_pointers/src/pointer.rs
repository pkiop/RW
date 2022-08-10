/*

    pointer : variable that conttains an address in memory

    type of pointer.
        reference : most common kind of pointer in rust. & symbol
                    no overhead, no any special capabilities
        Smart pointers : act like a pointer but also have additional metadata and capabilities
            keyword : reference counting.
                      smart pointer type
            concept : rust has ownership model. Smart pointer can own data they point to.
            few example : String, Vec<T> <- they own their memory, and it's pointer
            implement : using structs. -> implement the Deref and Drop

            Box<T> : for allocating values on the heap
            Rc<T> : a reference counting type that enables multiple ownership
            Ref<T>. RefMut<T>. RefCell<T> : enforce borrowing rules at runtime. instead of compile time

*/

/*
    Box<T> : Point to Data on the Heap

    The mose straightforward smart pointer.
    No performance overhead, other then storing their data on the heap instead of on the stack
    But they don't have many extra capabilities either.

    use case
    * size can't be known at compile time, need exact size
    * when have large amount of data, want to transfer ownership but ensure the data won't be copied when you do so
    * when want to own a value, only care type that a particular trait rather than being of a specific type


*/

enum List {
    Cons(i32, Box<List>), // Box -> size is usize (pointer size)
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5); // points to the value 5 // it allocated on the heap
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
