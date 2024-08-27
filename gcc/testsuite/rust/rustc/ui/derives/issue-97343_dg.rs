use std::fmt::Debug;

#[derive(Debug)]
pub struct Irrelevant<Irrelevant> { // { dg-error ".E0109." "" { target *-*-* } }
    irrelevant: Irrelevant,
}

fn main() {}

