#![feature(never_type)]
#![deny(unreachable_patterns)]
// { dg-note "" "" { target *-*-* } .-1 }

#[rustfmt::skip]
fn main() {
    match (0u8,) {
        (1 | 2,) => {}
// { dg-note "" "" { target *-*-* } .-1 }
        (2,) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        _ => {}
    }

    match (0u8,) {
        (1,) => {}
// { dg-note "" "" { target *-*-* } .-1 }
        (2,) => {}
// { dg-note "" "" { target *-*-* } .-1 }
        (1 | 2,) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
        _ => {}
    }

    match 0u8 {
        1 => {}
// { dg-note "" "" { target *-*-* } .-1 }
        2 => {}
// { dg-note "" "" { target *-*-* } .-1 }
        3 => {}
// { dg-note "" "" { target *-*-* } .-1 }
        4 => {}
// { dg-note "" "" { target *-*-* } .-1 }
        5 => {}
        6 => {}
        1 ..= 6 => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
        _ => {}
    }

    let res: Result<(),!> = Ok(());
    match res {
        Ok(_) => {}
        Err(_) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    }

    #[derive(Copy, Clone)]
    enum Void1 {}
    #[derive(Copy, Clone)]
    enum Void2 {}
    // Only an empty type matched _by value_ can make an arm unreachable. We must get the right one.
    let res1: Result<(), Void1> = Ok(());
    let res2: Result<(), Void2> = Ok(());
    match (&res1, res2) {
        (Err(_), Err(_)) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
        _ => {}
    }
    match (res1, &res2) {
        (Err(_), Err(_)) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
        _ => {}
    }


    if let (0
// { dg-note "" "" { target *-*-* } .-1 }
        | 0, _) = (0, 0) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

    match (true, true) {
        (_, true) if false => {} // Guarded patterns don't cover others
        (true, _) => {}
// { dg-note "" "" { target *-*-* } .-1 }
        (false, _) => {}
// { dg-note "" "" { target *-*-* } .-1 }
        (_, true) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
    }

    match (true, true) {
        (true, _) => {}
// { dg-note "" "" { target *-*-* } .-1 }
        (false, _) => {}
        #[allow(unreachable_patterns)]
        (_, true) => {} // Doesn't cover below because it's already unreachable.
        (true, true) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    }

    // Despite skipping some irrelevant cases, we still report a set of rows that covers the
    // unreachable one.
    match (true, true, 0) {
        (true, _, _) => {}
        (_, true, 0..10) => {}
// { dg-note "" "" { target *-*-* } .-1 }
        (_, true, 10..) => {}
        (_, true, 3) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        _ => {}
    }
}

