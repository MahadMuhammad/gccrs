macro_rules! pat {
    () => { Some(_) }
}

fn main() {
    match Some(false) {
        Some(_)
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
    match Some(false) {
        Some(_)
        _ => {}
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match Some(false) {
        Some(_),
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
    }
    match Some(false) {
        Some(_),
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        _ => {}
    }
    match Some(false) {
        Some(_) if true
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
    match Some(false) {
        Some(_) if true
        _ => {}
// { dg-error "" "" { target *-*-* } .-1 }
    }
    match Some(false) {
        Some(_) if true,
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
    match Some(false) {
        Some(_) if true,
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
    match Some(false) {
        pat!()
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
    match Some(false) {
        pat!(),
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
    match Some(false) {
        pat!() if true,
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    }
    match Some(false) {
        pat!()
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
    match Some(false) {
        pat!(),
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        _ => {}
    }
}

