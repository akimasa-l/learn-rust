fn parse() {
    println!("function \"parse\" was called.");
    let x: i64 = "100".parse().unwrap(); //本来だったらxの型を決めるためにxに型を決めてあげなきゃいけないけど、
    println!("x is {}", x);
}
fn main() {
    println!("Hello, world!");
    func();
    let x = 100;
    parse();
    let y = "600".parse().expect("Not a number!"); //次のfunc_with_argでyがi64として呼ばれているのでyは自動的にi64になる←すごくね！？
    func_with_arg(x, y);
}
fn func() {
    println!("function \"func\" was called.")
}
fn func_with_arg(x: i64, y: i64) {
    println!("function \"func_with_arg\" was called.");
    println!("x is {}", x);
    println!("y is {}", y);
}
