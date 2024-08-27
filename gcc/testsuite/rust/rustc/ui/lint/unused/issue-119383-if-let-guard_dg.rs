#![feature(if_let_guard)]
#![deny(unused_variables)]

fn main() {
    match () {
        () if let Some(b) = Some(()) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

