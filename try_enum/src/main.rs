fn main() {
    println!("Hello, world!");
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    kind(four);
    kind(six)
}
#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}
fn kind(k: IPAddrKind) {
    println!("{:?}", k)
}
