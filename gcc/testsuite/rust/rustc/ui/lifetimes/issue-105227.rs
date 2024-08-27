// Regression test for issue #105227.

// FIXME(precise_capturing): Add rustfix here after dealing w/ elided lifetimes

#![allow(unused)]

fn chars0(v :(& str, &str)) -> impl Iterator<Item = char> {
// { help "" "" { target *-*-* } .-1 }
    v.0.chars().chain(v.1.chars())
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn chars1(v0 : & str, v1 : &str) -> impl Iterator<Item = char> {
// { help "" "" { target *-*-* } .-1 }
    v0.chars().chain(v1.chars())
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn chars2<'b>(v0 : &str, v1 : &'_ str, v2 : &'b str) -> (impl Iterator<Item = char>, &'b str) {
// { help "" "" { target *-*-* } .-1 }
    (v0.chars().chain(v1.chars()), v2)
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn main() {
}

