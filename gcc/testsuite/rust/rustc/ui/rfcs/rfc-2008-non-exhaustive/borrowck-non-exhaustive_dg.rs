// Test that the borrow checker considers `#[non_exhaustive]` when checking
// whether a match contains a discriminant read.

//@ aux-build:monovariants.rs
extern crate monovariants;

use monovariants::NonExhaustiveMonovariant;

fn main() {
    let mut x = NonExhaustiveMonovariant::Variant(1);
    let y = &mut x;
    match x {
// { dg-error ".E0503." "" { target *-*-* } .-1 }
        NonExhaustiveMonovariant::Variant(_) => {},
        _ => {},
    }
    drop(y);
}

