// Check that never patterns can't have bodies or guards.
#![feature(never_patterns)]
#![allow(incomplete_features)]

enum Void {}

fn main() {}

macro_rules! never {
    () => { ! }
}

fn no_arms_or_guards(x: Void) {
    match &None::<Void> {
        Some(!) => {}
// { dg-error "" "" { target *-*-* } .-1 }
        None => {}
    }
    match &None::<Void> { // { dg-error ".E0004." "" { target *-*-* } }
        Some(!) if true,
// { dg-error "" "" { target *-*-* } .-1 }
        None => {}
    }
    match &None::<Void> { // { dg-error ".E0004." "" { target *-*-* } }
        Some(!) if true => {}
// { dg-error "" "" { target *-*-* } .-1 }
        None => {}
    }
    match &None::<Void> {
        Some(never!()) => {}
// { dg-error "" "" { target *-*-* } .-1 }
        None => {}
    }
}

