use std::collections::HashMap;
pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 20);

    println!("scores : {:?}", scores);
}
