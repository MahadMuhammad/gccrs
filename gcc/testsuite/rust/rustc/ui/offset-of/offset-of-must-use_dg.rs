//@ check-pass

#![warn(unused)]

fn main() {
    core::mem::offset_of!((String,), 0);
// { dg-warning "" "" { target *-*-* } .-1 }
}

