#![feature(deref_patterns)]
#![allow(incomplete_features)]

fn main() {
    // FIXME(deref_patterns): fails to typecheck because `"foo"` has type &str but deref creates a
    // place of type `str`.
    match "foo".to_string() {
        deref!("foo") => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        _ => {}
    }
    match &"foo".to_string() {
        deref!("foo") => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        _ => {}
    }
}

