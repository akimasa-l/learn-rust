fn main() {
    let user = User {
        mail: String::from("user@example.com"),
        name: String::from("usr"),
        age: 10,
    };
    println!("name is {}", user.name);
    println!("mail is {}", user.mail);
    println!("age is {}", user.age);

    let u = make_user(String::from("u@example.com"), 18);
    println!("name is {}", u.name);
    println!("mail is {}", u.mail);
    println!("age is {}", u.age);
    color();
}
struct User {
    mail: String,
    age: i8,
    name: String,
}
fn make_user(mail_ad: String, age: i8) -> User {
    User {
        mail: mail_ad,
        age,
        name: String::from("user100"),
    }
}
struct Color(i32, i32, i32);
fn color() {
    let color = Color(0xAC, 0x10, 0x00);
    println!("color is {}, {}, {}", color.0, color.1, color.2);
}
