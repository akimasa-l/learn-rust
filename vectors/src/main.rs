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
}
