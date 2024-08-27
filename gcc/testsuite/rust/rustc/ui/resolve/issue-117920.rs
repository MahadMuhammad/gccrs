#![crate_type = "lib"]

use super::A; // { dg-error ".E0433." "" { target *-*-* } }

mod b {
    pub trait A {}
    pub trait B {}
}

/// [`A`]
pub use b::*;

