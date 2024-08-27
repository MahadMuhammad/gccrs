#![feature(deref_patterns)]
#![allow(incomplete_features)]

#[rustfmt::skip]
fn main() {
    let mut b = Box::new(false);
    match b {
        deref!(true) => {}
        _ if { *b = true; false } => {}
// { dg-error ".E0510." "" { target *-*-* } .-1 }
        deref!(false) => {}
        _ => {},
    }
}

