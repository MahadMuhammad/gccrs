// Regression test for #88653, where a confusing warning about a
// type mismatch in coroutine arguments was issued.

#![feature(coroutines, coroutine_trait)]

use std::ops::Coroutine;

fn foo(bar: bool) -> impl Coroutine<(bool,)> {
// { dg-error ".E0631." "" { target *-*-* } .-1 }
// { dg-note ".E0631." "" { target *-*-* } .-2 }
// { dg-note ".E0631." "" { target *-*-* } .-3 }
// { dg-note ".E0631." "" { target *-*-* } .-4 }
// { dg-note ".E0631." "" { target *-*-* } .-5 }
    #[coroutine]
    |bar| {
// { dg-note "" "" { target *-*-* } .-1 }
        if bar {
            yield bar;
        }
    }
}

fn main() {}

