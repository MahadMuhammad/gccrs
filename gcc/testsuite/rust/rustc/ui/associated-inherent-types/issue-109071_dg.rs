//@ revisions: with_gate no_gate
#![cfg_attr(with_gate, feature(inherent_associated_types))]
#![cfg_attr(with_gate, allow(incomplete_features))]

struct Windows<T> { t: T }

impl<T> Windows { // { dg-error "" "" { target *-*-* } }
    type Item = &[T]; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

    fn next() -> Option<Self::Item> {}
}

impl<T> Windows<T> {
    fn T() -> Option<Self::Item> {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

