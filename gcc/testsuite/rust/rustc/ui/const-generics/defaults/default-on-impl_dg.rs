struct Foo<const N: usize>;

impl<const N: usize = 1> Foo<N> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

