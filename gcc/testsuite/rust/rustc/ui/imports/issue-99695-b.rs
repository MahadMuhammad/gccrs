//@ run-rustfix
#![allow(unused, nonstandard_style)]
mod m {

    mod p {
        #[macro_export]
        macro_rules! nu {
            {} => {};
        }

        pub struct other_item;
    }

    pub use self::p::{nu, other_item as _};
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { help ".E0432." "" { target *-*-* } .-2 }
}

fn main() {}

