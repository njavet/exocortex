mod guess;
mod rok;


fn main() {
    let s = String::from("this is a statement.");
    let word = rok::base::first_word_0(&s);
    println!("first word: {}", word);
    let word = rok::base::first_word_1(&s);
    println!("first word: {}", word);
}
