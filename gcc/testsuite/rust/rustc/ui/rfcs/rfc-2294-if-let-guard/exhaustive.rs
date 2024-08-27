#![feature(if_let_guard)]
#![allow(irrefutable_let_patterns)]

fn match_option(x: Option<u32>) {
    match x {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        Some(_) => {}
        None if let y = x => {}
    }
}

fn main() {
    let x = ();
    match x {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        y if let z = y => {}
    }
}

