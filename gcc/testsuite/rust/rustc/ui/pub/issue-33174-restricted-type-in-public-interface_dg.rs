//@ check-pass

#![allow(non_camel_case_types)] // genus is always capitalized

pub(crate) struct Snail;

mod sea {
    pub(super) struct Turtle;
}

struct Tortoise;

pub struct Shell<T> {
    pub(crate) creature: T,
}

pub type Helix_pomatia = Shell<Snail>;
// { dg-warning "" "" { target *-*-* } .-1 }
pub type Dermochelys_coriacea = Shell<sea::Turtle>;
// { dg-warning "" "" { target *-*-* } .-1 }
pub type Testudo_graeca = Shell<Tortoise>;
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {}

