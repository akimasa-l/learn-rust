fn main() {
    let coin=Coin::Penny;
    value_in_cent(coin);
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cent(coin: Coin) -> u32 {
    match coin {
        Penny => {
            println!("Lucky Penny!");
            1
        }
        Nickel => 5,
        Dime => 10,
        Quarter => 25,
    }
}
