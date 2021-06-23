use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"),50);
    let blue=scores.get("Blue");
    match blue{
        Some(x) => println!("scores Blue: {}", x),
        None=> println!("scores Blue is none")
    }
    
}
