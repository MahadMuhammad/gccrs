// Checks that lifetimes cannot be interspersed between consts and types.

struct Foo<const N: usize, 'a, T = u32>(&'a (), T);
// { dg-error "" "" { target *-*-* } .-1 }

struct Bar<const N: usize, T = u32, 'a>(&'a (), T);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

