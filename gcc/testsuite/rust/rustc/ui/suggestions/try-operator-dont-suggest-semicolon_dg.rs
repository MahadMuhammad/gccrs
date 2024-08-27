// Regression test for #87051, where a double semicolon was erroneously
// suggested after a `?` operator.

fn main() -> Result<(), ()> {
    a(|| {
// { help "" "" { target *-*-* } .-1 }
        b()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
    })?;

    // Here, we do want to suggest a semicolon:
    let x = Ok(42);
    if true {
// { dg-note "" "" { target *-*-* } .-1 }
        x?
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
    }
// { help "" "" { target *-*-* } .-1 }

    Ok(())
}

fn a<F>(f: F) -> Result<(), ()> where F: FnMut() { Ok(()) }
fn b() -> i32 { 42 }

