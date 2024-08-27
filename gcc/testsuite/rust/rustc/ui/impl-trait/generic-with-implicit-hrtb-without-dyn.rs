//@ revisions: edition2015 edition2021
// { dg-additional-options "-frust-edition=2021" }

#![allow(warnings)]

fn ice() -> impl AsRef<Fn(&())> {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-2 }
    todo!()
}

fn main() {}

