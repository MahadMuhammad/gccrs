//@ revisions: item local region

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

#[derive(Clone, Copy)]
pub enum Ty {}

impl Ty {
    type Pr<T: Copy> = T;

    type Static<Q: 'static> = Q;
}

#[cfg(item)]
const _: Ty::Pr<String> = String::new(); // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    #[cfg(local)]
    let _: Ty::Pr<Vec<()>>; // { dg-error "" "" { target *-*-* } }
}

fn user<'a>() {
    #[cfg(region)]
    let _: Ty::Static<&'a str> = ""; // { dg-error "" "" { target *-*-* } }
}

