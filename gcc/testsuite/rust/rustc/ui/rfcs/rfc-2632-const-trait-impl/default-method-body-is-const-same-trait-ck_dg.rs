#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

#[const_trait]
pub trait Tr {
    fn a(&self) {}

    fn b(&self) {
        ().a()
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }
}

impl Tr for () {}

fn main() {}

