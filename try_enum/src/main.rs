fn main() {
    println!("Hello, world!");
    let four=IPAddrKind::V4;
    let six=IPAddrKind::V6;
    println!("{:?} {:?}",four,six)
}
#[derive(Debug)]
enum IPAddrKind{
    V4,V6
}