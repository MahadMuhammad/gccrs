use std::fmt::Debug;

fn make_dyn_star() {
    let i = 42usize;
    let dyn_i: dyn* Debug = i as dyn* Debug;
// { dg-error ".E0606." "" { target *-*-* } .-1 }
// { dg-error ".E0606." "" { target *-*-* } .-2 }
// { dg-error ".E0606." "" { target *-*-* } .-3 }
}

fn main() {
    make_dyn_star();
}

