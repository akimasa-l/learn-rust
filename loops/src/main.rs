use proconio::input;
//use std::io::{stdout};
use std::io::{stdout, Write};
fn main() {
    loop {
        input! {
            n:i32
        };
        stdout().flush().unwrap();
        println!("n is {}", n);
    }
}
