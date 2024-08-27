// When denying at the crate level, be sure to not get random warnings from the
// injected intrinsics by the compiler.

#![deny(unused_attributes)]

mod a {
    #![crate_type = "bin"] // { dg-error "" "" { target *-*-* } }
}

#[crate_type = "bin"] fn main() {} // { dg-error "" "" { target *-*-* } }

