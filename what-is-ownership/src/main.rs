fn main() {
    let mut message = String::from("Hello");
    println!("message is: {}", message);
    message.push_str(", world!");
    println!("message is: {}", message)
}
