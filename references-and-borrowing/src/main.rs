fn main() {
    let mut s = String::from("apple");
    println!("s is {}", s);
    func(&s);
    println!("s is {}", s);
    change(&mut s);
    println!("s is {}", s);
}
fn func(s: &String) {
    println!("function \"func\" was called.");
    println!("length of s is {}", s.len());
}
fn change(s: &mut String) {
    s.push_str(" aa")
}
