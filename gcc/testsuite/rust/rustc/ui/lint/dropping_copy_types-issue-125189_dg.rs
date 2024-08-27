//@ check-fail
//@ run-rustfix

#![deny(dropping_copy_types)]

fn main() {
    let y = 1;
    drop(3.2); // { dg-error "" "" { target *-*-* } }
    drop(y); // { dg-error "" "" { target *-*-* } }
}

