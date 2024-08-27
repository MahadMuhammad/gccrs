//@ compile-flags: -Z unpretty=thir-tree

pub fn main() {
    break; // { dg-error ".E0268." "" { target *-*-* } }
}

