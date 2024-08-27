#![feature(type_alias_impl_trait)]

type Tait = impl Copy;
// Make sure that this TAIT isn't considered unconstrained...

fn empty_opaque() -> Tait {
    if false {
        match empty_opaque() {}
// { dg-error ".E0004." "" { target *-*-* } .-1 }
    }
    0u8
}

fn main() {}

