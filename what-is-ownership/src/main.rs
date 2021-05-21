fn main() {
    let mut message = String::from("Hello");
    println!("message is: {}", message);
    message.push_str(", world!");
    println!("message is: {}", message);
    let new_message = message;
    println!("new message is: {}", new_message);
    // println!("message is: {}",message);
    // コンパイルエラー
    try_clone();
    int();
}
fn try_clone() {
    let a = String::from("apple");
    println!("a is: {}", a);
    let b = a.clone();
    println!("a, b is : {}, {}", a, b)
}
fn int(){
    let a=5;
    let b=a;
    let c=a.clone();
    println!("a, b, c is : {}, {}, {}", a, b, c);
}