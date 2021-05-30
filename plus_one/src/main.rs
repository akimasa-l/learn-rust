fn main() {
    let x=Some(6);
    plus_one(x);
    plus_one(plus_one(x));
    let y=None;
    plus_one(y);
    plus_one(plus_one(y));
}
fn plus_one(x:Option<u8>)->Option<u8> {
    match x{
        Some(x) => {println!("x is: {}",x);Some(x+1)},
        None => { println!("x is None");None}
    }
}