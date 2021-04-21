use std::io;
fn main() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //println!("Hello, world!");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("return is {}", {
        let ans = if guess < 54 {
            println!("guess is lower than 54");
            54
        } else {
            println!("guess is larger or same to 54");
            43
        };
        ans
    })
}
