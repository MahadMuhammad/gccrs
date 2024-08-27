#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn b()
where
    for<const C: usize> [(); C]: Copy,
// { dg-error "" "" { target *-*-* } .-1 }
{
}

fn main() {}

