#![feature(never_patterns)]
#![allow(incomplete_features)]

enum Void {}

fn main() {}

macro_rules! never {
    () => { ! }
}

fn parse(x: Void) {
    match None::<Void> {
        None => {}
        Some(!),
    }
    match None::<Void> {
        Some(!),
        None => {}
    }
    match None::<Void> {
        None => {}
        Some(!)
    }
    match None::<Void> {
        Some(!)
// { dg-error "" "" { target *-*-* } .-1 }
        None => {}
    }
    match None::<Void> {
        Some(!) if true
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        None => {}
    }
    match None::<Void> {
        Some(!) if true,
// { dg-error "" "" { target *-*-* } .-1 }
        None => {}
    }
    match None::<Void> {
        Some(!) <=
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match x {
        never!(),
    }
    match x {
        never!() if true,
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match x {
        never!()
    }
    match &x {
        &never!(),
    }
    match None::<Void> {
        Some(never!()),
        None => {}
    }
    match x { ! }
    match &x { &! }

    let res: Result<bool, Void> = Ok(false);
    let Ok(_) = res;
    let Ok(_) | Err(!) = &res; // Disallowed; see #82048.
// { dg-error "" "" { target *-*-* } .-1 }
    let (Ok(_) | Err(!)) = &res;
    let (Ok(_) | Err(&!)) = res.as_ref();

    let ! = x;
    let y @ ! = x;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn foo(!: Void) {}

