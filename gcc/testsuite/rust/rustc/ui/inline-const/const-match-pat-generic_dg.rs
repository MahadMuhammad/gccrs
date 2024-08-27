#![feature(inline_const_pat)]

// rust-lang/rust#82518: ICE with inline-const in match referencing const-generic parameter

fn foo<const V: usize>() {
    match 0 {
        const { V } => {},
// { dg-error ".E0158." "" { target *-*-* } .-1 }
        _ => {},
    }
}

const fn f(x: usize) -> usize {
    x + 1
}

fn bar<const V: usize>() {
    match 0 {
        const { f(V) } => {},
// { dg-error ".E0158." "" { target *-*-* } .-1 }
        _ => {},
    }
}

fn main() {
    foo::<1>();
    bar::<1>();
}

