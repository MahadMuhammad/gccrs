//@ check-fail

#![deny(dropping_copy_types)]

fn main() {
    let y = 1;
    let z = 2;
    match y {
        0 => drop(y), // { dg-error "" "" { target *-*-* } }
        1 => drop(z), // { dg-error "" "" { target *-*-* } }
        2 => drop(3), // { dg-error "" "" { target *-*-* } }
        _ => {},
    }
}

