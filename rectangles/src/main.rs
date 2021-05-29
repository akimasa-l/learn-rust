fn main() {
    let r = Rectangle {
        width: 50,
        height: 30,
    };
    println!("r's width is {} and r's height is {}", r.width, r.height);
    //println!("r is {:?}",r);
    println!("area is {}", r.area());
}
//#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle{

fn area(self: &Rectangle) -> u32 {
    self.width * self.height
}
}
