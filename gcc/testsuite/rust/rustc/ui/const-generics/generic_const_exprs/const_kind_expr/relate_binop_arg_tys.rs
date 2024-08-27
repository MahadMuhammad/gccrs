// checks that when we relate a `Expr::Binop` we also relate the types of the
// const arguments.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Bar<const B: bool>;

const fn make_generic(_: usize, a: bool) -> bool {
    a
}

fn foo<const N: usize>() -> Bar<{ make_generic(N, true == false) }> {
    Bar::<{ make_generic(N, 1_u8 == 0_u8) }>
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

fn main() {}

