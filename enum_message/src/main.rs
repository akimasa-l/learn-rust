fn main() {
    println!("Hello, world!");
    let m = Message::Write(String::from("apple"));
    m.call()
}
impl Message {
    fn call(&self) {
        println!("function was called.")
    }
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
