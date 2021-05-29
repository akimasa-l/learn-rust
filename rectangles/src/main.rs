fn main() {
    let r = Rectangle {
        width: 50,
        height: 30,
    };
    println!("r's width is {} and r's height is {}", r.width, r.height);
    println!("r is {:?}",r);
    println!("area is {}", area(&r));
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
