struct A<T = u32, const N: usize> {
// { dg-error "" "" { target *-*-* } .-1 }
    arg: T,
}

struct Foo<const N: u8 = 3, T>(T);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

