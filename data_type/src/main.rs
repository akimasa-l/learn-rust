fn array() {
    let mut a = [1, 4, 6];
    a[2]=4;
    println!("a is {:?}", a);
    println!("a[0] is {}", a[0]);
    let b = a[0] + 10;
    println!("b is {}", b);
    let index:usize = "10".parse().unwrap();
    println!("a[index] is {}", a[index]);
}
fn double() {
    let a = 100000.15;
    println!("a = {}", a);
    let a: f32 = a;
    println!("a = {}", a);
}
fn main() {
    let guess
    :isize//ここをコメントアウトしてみる
    = "42".parse()
    .expect("Not a number!");
    // println!("Hello, world!");
    println!("guess is {}", guess);
    double();
    array();
}
