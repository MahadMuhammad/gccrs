// regression test for #74018

#![feature(impl_trait_in_assoc_type)]

trait Trait {
    type Associated;
    fn into(self) -> Self::Associated;
}

impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
    type Associated = (i32, impl Iterator<Item = i32>);
    fn into(self) -> Self::Associated {
        (0_i32, [0_i32].iter().copied())
// { dg-error ".E0792." "" { target *-*-* } .-1 }
    }
}

fn main() {}

