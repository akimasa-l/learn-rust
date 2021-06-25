use proconio::input;
use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(String::from("init"), 0);
    loop{
    input! {
        t:i32,
        s:String,
        a:i32,
    }
    match t {
        0 => {
            map.insert(s, a);
        }

        1 => {
            let result = map.get(&s);
            match result {
                Some(x) => println!("map result: {} is {}",&s, x),
                None => println!("map result is none"),
            };
        }
        _ => {
            println!("Please type \"insert\" or \"get\"");
            break;
        }
    }
}
}