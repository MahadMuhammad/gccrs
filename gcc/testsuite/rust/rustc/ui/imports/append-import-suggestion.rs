// https://github.com/rust-lang/rust/issues/114884

mod mod1 {
    pub trait TraitA {}
}

mod mod2 {
    mod sub_mod {
       use super::super::mod1::TraitA;
    }
}

use mod2::{sub_mod::TraitA};
// { dg-error ".E0603." "" { target *-*-* } .-1 }

fn main() {}

