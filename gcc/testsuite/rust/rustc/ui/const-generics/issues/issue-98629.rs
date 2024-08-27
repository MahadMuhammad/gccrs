#![feature(const_trait_impl)]

#[const_trait]
trait Trait {
    const N: usize;
}

impl const Trait for i32 {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

fn f()
where
    [(); <i32 as Trait>::N]:,
{}

fn main() {}

