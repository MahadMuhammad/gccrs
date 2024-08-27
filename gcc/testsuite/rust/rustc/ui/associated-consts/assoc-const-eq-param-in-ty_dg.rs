// Regression test for issue #108271.
// Detect and reject generic params in the type of assoc consts used in an equality bound.
#![feature(associated_const_equality)]

trait Trait<'a, T: 'a, const N: usize> {
    const K: &'a [T; N];
}

fn take0<'r, A: 'r, const Q: usize>(_: impl Trait<'r, A, Q, K = { loop {} }>) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { dg-note "" "" { target *-*-* } .-7 }
// { dg-note "" "" { target *-*-* } .-8 }
// { dg-error "" "" { target *-*-* } .-9 }
// { dg-note "" "" { target *-*-* } .-10 }
// { dg-note "" "" { target *-*-* } .-11 }
// { dg-note "" "" { target *-*-* } .-12 }

trait Project {
    const SELF: Self;
}

fn take1(_: impl Project<SELF = {}>) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }

fn take2<P: Project<SELF = {}>>(_: P) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

trait Iface<'r> {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    type Assoc<const Q: usize>: Trait<'r, Self, Q, K = { loop {} }>
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { dg-note "" "" { target *-*-* } .-7 }
// { dg-error "" "" { target *-*-* } .-8 }
// { dg-note "" "" { target *-*-* } .-9 }
// { dg-note "" "" { target *-*-* } .-10 }
// { dg-note "" "" { target *-*-* } .-11 }
// { dg-note "" "" { target *-*-* } .-12 }
// { dg-note "" "" { target *-*-* } .-13 }
// { dg-error "" "" { target *-*-* } .-14 }
// { dg-note "" "" { target *-*-* } .-15 }
// { dg-note "" "" { target *-*-* } .-16 }
// { dg-error "" "" { target *-*-* } .-17 }
// { dg-note "" "" { target *-*-* } .-18 }
// { dg-note "" "" { target *-*-* } .-19 }
// { dg-note "" "" { target *-*-* } .-20 }
// { dg-note "" "" { target *-*-* } .-21 }
// { dg-note "" "" { target *-*-* } .-22 }
// { dg-note "" "" { target *-*-* } .-23 }
    where
        Self: Sized + 'r;
}

fn main() {}

