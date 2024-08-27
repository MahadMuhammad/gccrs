// Regression test for #69615.

#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

#[const_trait]
pub trait MyTrait {
    fn method(&self) -> Option<()>;
}

impl const MyTrait for () {
    fn method(&self) -> Option<()> {
        Some(())?; // { dg-error ".E0658." "" { target *-*-* } }
        None
    }
}

fn main() {}

