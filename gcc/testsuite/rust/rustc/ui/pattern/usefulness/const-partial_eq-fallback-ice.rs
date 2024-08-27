#![allow(warnings)]

struct MyType;

impl PartialEq<usize> for MyType {
    fn eq(&self, y: &usize) -> bool {
        true
    }
}

const CONSTANT: &&MyType = &&MyType;

fn main() {
    if let CONSTANT = &&MyType {
// { dg-error "" "" { target *-*-* } .-1 }
        println!("did match!");
    }
}

