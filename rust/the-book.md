### ch11

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


### chapter07
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





