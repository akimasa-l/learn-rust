fn main() {
    let area1 = Rectangle {
        width: 50,
        height: 30,
    };
    println!(
        "area1's width is {} and area1's height is {}",
        area1.width, area1.height
    );
    //println!("r is {:?}",r);
    println!("area is {}", area1.area());
    let area2 = Rectangle {
        width: 20,
        height: 10,
    };
    if area1.can_hold(&area2) {
        println!("area1 can hold area2");
    } else {
        println!("area1 cannot hold area2");
    };
    let area3 = Rectangle::square(20);
    println!("area3's area is {}", area3.area());
}
//#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(self: &Rectangle) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
