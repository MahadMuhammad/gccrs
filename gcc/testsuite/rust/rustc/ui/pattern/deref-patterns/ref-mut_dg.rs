#![feature(deref_patterns)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::rc::Rc;

fn main() {
    match &mut vec![1] {
        deref!(x) => {}
        _ => {}
    }

    match &mut Rc::new(1) {
        deref!(x) => {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        _ => {}
    }
}

