// Check that a suggestion is issued if there are too many `<`s in a
// generic argument list, and that the parser recovers properly.

fn main() {
    foo::<<<<Ty<i32>>();
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
// { dg-error ".E0425." "" { target *-*-* } .-3 }
}

