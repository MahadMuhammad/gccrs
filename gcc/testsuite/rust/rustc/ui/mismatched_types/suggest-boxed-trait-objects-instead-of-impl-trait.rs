//@ run-rustfix

#![allow(dead_code)]

struct S;
struct Y;

trait Trait {}

impl Trait for S {}
impl Trait for Y {}

fn foo() -> impl Trait {
    if true {
        S
    } else {
        Y // { dg-error ".E0308." "" { target *-*-* } }
    }
}

fn bar() -> impl Trait {
    match true {
        true => S,
        false => Y, // { dg-error ".E0308." "" { target *-*-* } }
    }
}

fn main() {}

