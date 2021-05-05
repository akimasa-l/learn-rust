use proconio::input;
use std::io::{stdout};
fn main() {
    loop {
        input! {
            n:i32
        };
        stdout().flush().unwrap();
        println!("n is {}", n);
    }
}
