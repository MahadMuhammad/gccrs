//@ check-fail
//@ run-rustfix

#![deny(dropping_copy_types)]

use std::fmt::Write;

fn main() {
    let mut msg = String::new();
    drop(writeln!(&mut msg, "test"));
// { dg-error "" "" { target *-*-* } .-1 }
}

