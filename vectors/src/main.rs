fn main() {
    let mut l = vec![2, 3, 5];
    println!("first element is {}", l[0]);
    l.push(1);
    let second: Option<&i32> = l.get(/* 2 */ 100);
    if let Some(x) = second {
        println!("second element is {}", x);
    } else {
        println!("second element doesn't exist");
    }
    for i in &mut l {
        *i *= 4;
        println!("{}", i);
    }
    let cells = vec![
        SpreadSheetCell::Text(String::from("apple")),
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(3.5),
    ];
    for i in &cells {
        i.print();
    }
}
enum SpreadSheetCell {
    Text(String),
    Int(i32),
    Float(f64),
}
impl SpreadSheetCell {
    fn print(&self) {
        match self {
            SpreadSheetCell::Text(s) => {
                println!("This is String");
                println!("{}", s)
            }
            SpreadSheetCell::Int(x) => {
                println!("This is i32");
                println!("{}", x)
            }
            SpreadSheetCell::Float(y) => {
                println!("This is f64");
                println!("{}", y)
            }
        }
    }
}
