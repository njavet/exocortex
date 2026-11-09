## The Rust Book 

### chap02
read line to string variable:

rust:
```
use std::io;

fn main() { 
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("failed to read line");
    println!("input: {}", inp);
}

```
python:
```
def main():
    inp = input()
    print("input", inp)
```


