//@ check-pass

#![warn(unused_unsafe)]

const unsafe fn require_unsafe() -> usize { 1 }

fn main() {
    unsafe {
        const {
            require_unsafe();
            unsafe {}
// { dg-warning "" "" { target *-*-* } .-1 }
        }
    }
}

