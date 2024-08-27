#![deny(unused_must_use)]

pub fn fun() -> i32 {
    function() && return 1;
// { dg-error "" "" { target *-*-* } .-1 }
    return 0;
}

fn function() -> bool {
    true
}

fn main() {}

