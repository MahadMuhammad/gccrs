#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn f()
where
    for<B> B::Item: Send,
// { dg-error ".E0223." "" { target *-*-* } .-1 }
{
}

fn main() {}

