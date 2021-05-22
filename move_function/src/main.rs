fn main() {
    let s = String::from("orange");
    println!("s is {}", s);
    call_for_string(s);
    //println!("s is {}",s);
    //コンパイルエラー

    let t = String::from("apple");
    println!("t is {}", t);
    call_for_string(t.clone());
    println!("t is {}", t);

    let i = 100;
    println!("i is {}", i);
    makes_copy(i);
    makes_copy(i.clone());
    println!("i is {}", i);
}
fn call_for_string(s: String) {
    println!("function call_for_string was called.");
    println!("s is {}", s);
}
fn makes_copy(i: i32) {
    println!("function make_copy was called.");
    println!("i is {}", i);
}
