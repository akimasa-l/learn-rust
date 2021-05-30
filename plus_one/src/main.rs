fn main() {
    let mut x = Some(6);
    plus_one(x);
    plus_one(plus_one(x));
    let y = None;
    plus_one(y);
    plus_one(plus_one(y));

    plus_one_smart(plus_one_smart(x));
    plus_one_smart(plus_one_smart(y));

    plus_one_smartest(&mut x);
}
fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        Some(x) => {
            println!("x is: {}", x);
            Some(x + 1)
        }
        None => {
            println!("x is None");
            None
        }
    }
}
fn plus_one_smart(x: Option<u8>) -> Option<u8> {
    if let Some(x) = x {
        println!("x is: {}", x);
        Some(x + 1)
    } else {
        println!("x is None");
        None
    }
}
fn plus_one_smartest(x: &mut Option<u8>) {
    if let Some(x) = x {
        *x += 1;
        println!("x is {}", x)
    }
}
