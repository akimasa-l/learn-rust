fn main() {
    let s = String::from("orange");
    println!("s is {}", s);
    call_for_string(s);
    //println!("s is {}",s);
    //コンパイルエラー

    let t= String::from("apple");
    println!("t is {}",t);
    call_for_string(t.clone());
    println!("t is {}",t)
}
fn call_for_string(s: String) {
    println!("function call_for_string was called.");
    println!("s is {}", s);
}
