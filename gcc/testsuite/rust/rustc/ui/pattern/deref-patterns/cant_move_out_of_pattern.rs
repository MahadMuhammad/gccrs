#![feature(deref_patterns)]
#![allow(incomplete_features)]

use std::rc::Rc;

struct Struct;

fn cant_move_out_box(b: Box<Struct>) -> Struct {
    match b {
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        deref!(x) => x,
        _ => unreachable!(),
    }
}

fn cant_move_out_rc(rc: Rc<Struct>) -> Struct {
    match rc {
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        deref!(x) => x,
        _ => unreachable!(),
    }
}

fn main() {}

