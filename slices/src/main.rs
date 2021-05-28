fn main() {
    let s = String::from("Hello, world!");
    println!("s is {}", s);
    let slice = &s[0..5];
    println!("slice is {}", slice);

    let slice = &s[1..10];
    println!("slice is {}", slice);
    let slice = &s[1..];
    println!("slice is {}", slice);

    let slice = &s[..10];
    println!("slice is {}", slice);

    let slice = &s[..];
    println!("slice is {}", slice);

    println!("first word is {}", first_word(&s[..]));
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
