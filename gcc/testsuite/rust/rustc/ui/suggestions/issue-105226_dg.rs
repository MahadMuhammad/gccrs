use std::fmt;

struct S {
}

impl S {
    fn hello<P>(&self, val: &P) where P: fmt::Display; {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        println!("val: {}", val);
    }
}

impl S {
    fn hello_empty<P>(&self, val: &P) where P: fmt::Display;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    let s = S{};
    s.hello(&32);
}

