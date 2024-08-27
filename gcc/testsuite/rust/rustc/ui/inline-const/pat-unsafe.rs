//@ check-pass

#![warn(unused_unsafe)]
#![feature(inline_const_pat)]

const unsafe fn require_unsafe() -> usize {
    1
}

fn main() {
    unsafe {
        match () {
            const {
                require_unsafe();
                unsafe {}
// { dg-warning "" "" { target *-*-* } .-1 }
            } => (),
        }

        match 1 {
            const {
                unsafe {}
// { dg-warning "" "" { target *-*-* } .-1 }
                require_unsafe()
            }..=4 => (),
            _ => (),
        }
    }
}

