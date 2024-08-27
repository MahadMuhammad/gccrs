#![feature(never_patterns)]
#![allow(incomplete_features)]

enum Void {}

fn main() {}

fn anything<T>() -> T {
    let x: Void;
    match x { ! }
// { dg-error ".E0381." "" { target *-*-* } .-1 }
}

