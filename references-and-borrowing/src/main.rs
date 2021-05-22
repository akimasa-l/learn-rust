fn main() {
    let s = String::from("apple");
    println!("s is {}", s);
    func(&s);
    println!("s is {}", s);
}
fn func(s: &String) {
    println!("function \"func\" was called.");
    println!("length of s is {}", s.len());
}
