//@ run-pass
#![allow(unused_imports)]
// { dg-additional-options "-frust-edition=2018" }

fn main() {
    use std;
    let std = "std";
    println!("{}", std);
}

