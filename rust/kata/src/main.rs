mod guess;
mod rok;


struct Rectangle {
    width: u8,
    height: u8,
}


fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rect width: {}", rect.width);
    println!("rect height: {}", rect.height);
}
