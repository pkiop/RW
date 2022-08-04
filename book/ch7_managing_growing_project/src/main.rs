mod garden;

// start with crate :: absolute path
// start with self or other module or super(like ..) :: relative path

use crate::garden::vegetables;

mod test {
    pub fn test_fn() {
        println!("run test module -> test_fn");
    }
}
fn main() {
    test::test_fn();
    garden::run();
    vegetables::run();

    println!("Hello, world!");
}
