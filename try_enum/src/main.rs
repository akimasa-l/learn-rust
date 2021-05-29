fn main() {
    println!("Hello, world!");
    let four = IPAddrKind::V4(127,0,0,1);
    let six= IPAddrKind::V6(String::from("0129480219380"));
    kind(four);
    kind(six);
}
#[derive(Debug)]
enum IPAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}
fn kind(k: IPAddrKind) {
    println!("{:?}", k)
}
