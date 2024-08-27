#![feature(unboxed_closures, tuple_trait)]

use std::io::Read;

fn to_fn_once<A:std::marker::Tuple,F:FnOnce<A>>(f: F) -> F { f }

fn main() {
    let x = 1;
    to_fn_once(move|| { x = 2; });
// { dg-error ".E0594." "" { target *-*-* } .-1 }

    let s = std::io::stdin();
    to_fn_once(move|| { s.read_to_end(&mut Vec::new()); });
// { dg-error ".E0596." "" { target *-*-* } .-1 }
}

