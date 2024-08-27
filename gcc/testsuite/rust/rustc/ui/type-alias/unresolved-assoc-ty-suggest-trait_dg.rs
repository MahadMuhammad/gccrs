// Ensure that we don't suggest *type alias bounds* for **eager** type aliases.
// issue: rust-lang/rust#125789

//@ revisions: eager lazy
#![cfg_attr(lazy, feature(lazy_type_alias), allow(incomplete_features))]

trait Trait { type Assoc; }

type AssocOf<T> = T::Assoc; // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

type AssokOf<T> = T::Assok; // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

trait Parametrized<'a, T, const N: usize> {
    type Proj;
}

type ProjOf<T> = T::Proj; // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {}

