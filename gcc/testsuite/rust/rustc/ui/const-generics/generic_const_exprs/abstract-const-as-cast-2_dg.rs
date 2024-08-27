//@ run-rustfix
#![feature(generic_const_exprs)]
#![allow(incomplete_features, dead_code)]

struct Evaluatable<const N: u128> {}

struct Foo<const N: u8>([u8; N as usize])
// { dg-error "" "" { target *-*-* } .-1 }
where
    Evaluatable<{N as u128}>:;
// { help "" "" { target *-*-* } .-1 }

struct Foo2<const N: u8>(Evaluatable::<{N as u128}>) where Evaluatable<{N as usize as u128 }>:;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

struct Bar<const N: u8>([u8; (N + 2) as usize]) where [(); (N + 1) as usize]:;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {}

