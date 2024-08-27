#![deny(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

pub enum E {
    A,
    B,
}

pub mod b {
    pub fn key(e: ::E) -> &'static str {
        match e {
            A => "A",
// { dg-error ".E0170." "" { target *-*-* } .-1 }
            B => "B", // { dg-error ".E0170." "" { target *-*-* } }
// { dg-error ".E0170." "" { target *-*-* } .-1 }
        }
    }
}

fn main() {}

