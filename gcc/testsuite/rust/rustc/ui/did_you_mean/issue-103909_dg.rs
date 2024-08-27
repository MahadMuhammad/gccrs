#![allow(unused_variables)]
use std::fs::File;

fn main() {
    if Err(err) = File::open("hello.txt") {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
}

