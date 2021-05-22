fn main() {
    let s = String::from("orange");
    println!("s is {}", s);
    call_for_String(s);
    //println!("s is {}",s);
}
fn call_for_String(s: String) {
    println!("function call_for_String was called.");
    println!("s is {}", s);
}
