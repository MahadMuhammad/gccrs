// Regression test for #88403, where prefixing with an underscore was
// erroneously suggested for a nested shorthand struct field binding.

//@ run-rustfix
#![allow(unused)]
#![forbid(unused_variables)]

struct Inner { i: i32 }
struct Outer { o: Inner }

fn foo(Outer { o: Inner { i } }: Outer) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {
    let s = Outer { o: Inner { i: 42 } };
    let Outer { o: Inner { i } } = s;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

