//@ check-pass
//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }

#![warn(rust_2021_prefixes_incompatible_syntax)]

macro_rules! m2 {
    ($a:tt $b:tt) => {};
}

macro_rules! m3 {
    ($a:tt $b:tt $c:tt) => {};
}

fn main() {
    m2!(z"hey");
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    m2!(prefix"hey");
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    m3!(hey#123);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    m3!(hey#hey);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

macro_rules! quote {
    (# name = # kind # value) => {};
}

quote! {
    #name = #kind#value
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

