
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

pub fn first_word_0(s: &String) -> String {
    let words: Vec<&str> = s.split(' ').collect();
    words[0].to_string()
}

pub fn first_word_1(s: &String) -> &str {
    for (index, &items) in s.as_bytes().iter().enumerate() {
        if items == b' ' {
            return &s[..index];
        }
    }
    s
}
