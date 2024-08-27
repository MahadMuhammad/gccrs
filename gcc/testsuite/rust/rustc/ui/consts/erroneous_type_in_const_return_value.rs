//! ICE test #124348
//! We should not be running const eval if the layout has errors.

enum Eek {
    TheConst,
    UnusedByTheConst(Sum),
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

const EEK_ZERO: &[Eek] = &[];

fn main() {}

