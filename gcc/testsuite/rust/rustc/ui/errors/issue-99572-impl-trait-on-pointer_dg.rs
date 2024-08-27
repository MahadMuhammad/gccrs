// Emit additional suggestion to correct the trait implementation
// on a pointer
use std::{fmt, marker};

struct LocalType;

impl fmt::Display for *mut LocalType {
// { dg-error ".E0117." "" { target *-*-* } .-1 }
// { dg-note ".E0117." "" { target *-*-* } .-2 }
// { dg-note ".E0117." "" { target *-*-* } .-3 }
// { dg-note ".E0117." "" { target *-*-* } .-4 }
// { help ".E0117." "" { target *-*-* } .-5 }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This not compile")
    }
}

impl<T> marker::Copy for *mut T {
// { dg-error ".E0117." "" { target *-*-* } .-1 }
// { dg-note ".E0117." "" { target *-*-* } .-2 }
// { dg-note ".E0117." "" { target *-*-* } .-3 }
// { dg-note ".E0117." "" { target *-*-* } .-4 }
}

fn main() {}

