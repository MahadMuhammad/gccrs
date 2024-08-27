//@ compile-flags: -Znext-solver

// Coherence should handle overflow while normalizing for
// `trait_ref_is_knowable` correctly.

trait Overflow {
    type Assoc;
}
impl<T> Overflow for T {
    type Assoc = <T as Overflow>::Assoc;
// { dg-error ".E0275." "" { target *-*-* } .-1 }
}


trait Trait {}
impl<T: Copy> Trait for T {}
struct LocalTy;
impl Trait for <LocalTy as Overflow>::Assoc {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() {}

