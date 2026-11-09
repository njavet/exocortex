# The Rust Book

--- 
## chap02
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

--- 
## chap03
### datatypes
* scalar: integer, float, boolean, character
* integer type range: -(2**(n-1)) to (2**(n-1)-1)

---
## chap04

---
## chap05

---
## chap06

---
## chap07
workspace -> packages -> crates -> modules -> files

#### crates
* a crate is the smallest amount of code that the rust compiler considers at a time. 
* a file can be a crate. (rustc main.rs)
* binary(must contain fn main())  or library (do not have a fn main())

#### package
* bundle of one or more crates
* contains Cargo.toml
* can contain many binary crates, but only one library crate (why not the opposite ? )
* must contain at least one crate
 
---
## chap08

---
## chap09

---
## chap10

---
## chap11

```
#[cft(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        assert!()
        assert_eq!()
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn bar() {

    }

}
```
`cargo test`
