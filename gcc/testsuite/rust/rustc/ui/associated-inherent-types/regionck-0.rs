#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct S<T>(T);

impl S<&'static ()> {
    type T = ();
}

fn user<'a>() {
    let _: S::<&'a ()>::T; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

