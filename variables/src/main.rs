fn sub() {
    let x = 5;
    println!("x = {}", x);
    let x = x + 1;
    println!("x = {}", x);
    let x = x + 3;
    println!("x = {}", x);
}
fn sub_2(){
    let spaces="    ";
    println!("spaces are \"{}\"", spaces);
    let spaces = spaces.len();
    println!("spaces are {}", spaces);
}
const MOD: i32 = 1_000_000_007;
fn main() {
    let mut a = 10;
    println!("a is {}", a);
    a = 100;
    println!("a is {}", a / 11);
    let b = (1, 4, 5);
    println!("b is {}", b.2);
    println!("apple");
    println!("MOD = {}", MOD);
    sub();
    sub_2();
}
