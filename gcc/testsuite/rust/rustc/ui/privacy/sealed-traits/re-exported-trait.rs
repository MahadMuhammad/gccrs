//@ run-rustfix
#![allow(dead_code)]

pub mod a {
    pub use self::b::Trait;
    mod b {
        pub trait Trait {}
    }
}

struct S;
impl a::b::Trait for S {} // { dg-error ".E0603." "" { target *-*-* } }

fn main() {}

