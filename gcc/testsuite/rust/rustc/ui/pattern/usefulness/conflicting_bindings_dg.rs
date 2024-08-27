#![feature(if_let_guard, let_chains)]

fn main() {
    let mut x = Some(String::new());
    let ref mut y @ ref mut z = x;
// { dg-error "" "" { target *-*-* } .-1 }
    let Some(ref mut y @ ref mut z) = x else { return };
// { dg-error "" "" { target *-*-* } .-1 }
    if let Some(ref mut y @ ref mut z) = x {}
// { dg-error "" "" { target *-*-* } .-1 }
    if let Some(ref mut y @ ref mut z) = x && true {}
// { dg-error "" "" { target *-*-* } .-1 }
    while let Some(ref mut y @ ref mut z) = x {}
// { dg-error "" "" { target *-*-* } .-1 }
    while let Some(ref mut y @ ref mut z) = x && true {}
// { dg-error "" "" { target *-*-* } .-1 }
    match x {
        ref mut y @ ref mut z => {} // { dg-error "" "" { target *-*-* } }
    }
    match () {
        () if let Some(ref mut y @ ref mut z) = x => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

