//@ run-pass

#![feature(dyn_star)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::fmt::Debug;

fn main() {
    let x: dyn* Debug = &42;
    let x = Box::new(x) as Box<dyn Debug>;
    assert_eq!("42", format!("{x:?}"));

    // Also test opposite direction.
    let x: Box<dyn Debug> = Box::new(42);
    let x = &x as dyn* Debug;
    assert_eq!("42", format!("{x:?}"));
}

