fn main() {
    let a = ["apples", "grapes", "oranges"];
    for i in a.iter() {
        println!("I like {}!", i);
    }
    countdown()
}
fn countdown() {
    for i in (0..5).rev() {
        println!("{}!", i);
    }
    println!("liftoff!")
}
