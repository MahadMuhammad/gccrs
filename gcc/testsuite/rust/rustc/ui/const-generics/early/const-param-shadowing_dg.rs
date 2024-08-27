type N = u32;
struct Foo<const M: usize>;
fn test<const N: usize>() -> Foo<N> { // { dg-error ".E0747." "" { target *-*-* } }
    Foo
}

fn main() {}

