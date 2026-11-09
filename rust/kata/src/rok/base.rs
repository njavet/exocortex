
pub fn stack_move() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let s0 = "yo";
    let s1 = s0;
    println!("s0: {}, s1: {}", s0, s1);
}

pub fn heap_move() {
    let s0 = String::from("en taro adun");
    let s1 = s0;
    println!("s0 not valid anymore...");
    println!("new string: {}", s1);
}
