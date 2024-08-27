// { dg-additional-options "-frust-edition= 2021" }

#![allow(unused)]

mod A {
    pub(crate) type AA = ();
    pub(crate) type BB = ();

    mod A2 {
        use super::{super::C::D::AA, AA as _};
// { dg-error ".E0432." "" { target *-*-* } .-1 }
    }
}

mod C {
    pub mod D {}
}

mod B {
    use crate::C::{self, AA};
// { dg-error ".E0432." "" { target *-*-* } .-1 }

    use crate::{A, C::BB};
// { dg-error ".E0432." "" { target *-*-* } .-1 }
}

fn main() {}

