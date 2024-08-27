//@ check-pass

#![allow(unused)]
#![warn(unreachable_pub)]

mod private_mod {
    // non-leaked `pub` items in private module should be linted
    pub use std::fmt; // { dg-warning "" "" { target *-*-* } }
    pub use std::env::{Args}; // braced-use has different item spans than unbraced
// { dg-warning "" "" { target *-*-* } .-1 }

    // we lint on struct definition
    pub struct Hydrogen { // { dg-warning "" "" { target *-*-* } }
        // but not on fields, even if they are `pub` as putting `pub(crate)`
        // it would clutter the source code for little value
        pub neutrons: usize,
        pub(crate) electrons: usize
    }
    pub(crate) struct Calcium {
        pub neutrons: usize,
    }
    impl Hydrogen {
        // impls, too
        pub fn count_neutrons(&self) -> usize { self.neutrons } // { dg-warning "" "" { target *-*-* } }
        pub(crate) fn count_electrons(&self) -> usize { self.electrons }
    }
    impl Clone for Hydrogen {
        fn clone(&self) -> Hydrogen {
            Hydrogen { neutrons: self.neutrons, electrons: self.electrons }
        }
    }

    pub enum Helium {} // { dg-warning "" "" { target *-*-* } }
    pub union Lithium { c1: usize, c2: u8 } // { dg-warning "" "" { target *-*-* } }
    pub fn beryllium() {} // { dg-warning "" "" { target *-*-* } }
    pub trait Boron {} // { dg-warning "" "" { target *-*-* } }
    pub const CARBON: usize = 1; // { dg-warning "" "" { target *-*-* } }
    pub static NITROGEN: usize = 2; // { dg-warning "" "" { target *-*-* } }
    pub type Oxygen = bool; // { dg-warning "" "" { target *-*-* } }

    macro_rules! define_empty_struct_with_visibility {
        ($visibility: vis, $name: ident) => { $visibility struct $name {} }
// { dg-warning "" "" { target *-*-* } .-1 }
    }
    define_empty_struct_with_visibility!(pub, Fluorine);

    extern "C" {
        pub fn catalyze() -> bool; // { dg-warning "" "" { target *-*-* } }
    }

    // items leaked through signatures (see `get_neon` below) are OK
    pub struct Neon {}

    // crate-visible items are OK
    pub(crate) struct Sodium {}
}

pub mod public_mod {
    // module is public: these are OK, too
    pub struct Magnesium {}
    pub(crate) struct Aluminum {}
}

pub fn get_neon() -> private_mod::Neon {
    private_mod::Neon {}
}

fn main() {
    let _ = get_neon();
}

