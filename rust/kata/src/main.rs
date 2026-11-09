mod guess;
mod rok;

fn append_zero(s: &String) -> String {
    let mut ns = String::from(s);
    ns.push_str("-zero");
    ns
}

fn main() {
    let s = String::from("one");
    let ns = append_zero(&s);
    println!("old string: {}", s);
    println!("new string: {}", ns);
    rok::base::stack_move();
}
