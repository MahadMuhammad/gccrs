// Effective visibility tracking for imports is fine-grained, so `S2` is not fully exported
// even if its parent import (`m::*`) is fully exported as a `use` item.

#![feature(rustc_attrs)]

mod m {
    #[rustc_effective_visibility]
    pub struct S1 {} // { dg-error "" "" { target *-*-* } }
    #[rustc_effective_visibility]
    pub struct S2 {} // { dg-error "" "" { target *-*-* } }
}

mod glob {
    #[rustc_effective_visibility]
    pub use crate::m::*; // { dg-error "" "" { target *-*-* } }
}

#[rustc_effective_visibility]
pub use glob::S1; // { dg-error "" "" { target *-*-* } }

fn main() {}

