struct Ok {
    mk: String,
}
fn main() {
    let mut a = 1000;
    a += 1;
    let ok = Ok {
        mk: String::from("apple"),
    };
    println!("{}", a * a);
    println!("Hello, world!");
    println!("{}", ok.mk)
}
