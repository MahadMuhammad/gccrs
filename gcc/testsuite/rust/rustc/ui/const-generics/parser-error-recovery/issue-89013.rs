trait Foo<const N: usize> {
    fn do_x(&self) -> [u8; N];
}

struct Bar;

const T: usize = 42;

impl Foo<N = const 3> for Bar {
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
// { dg-error ".E0229." "" { target *-*-* } .-3 }
    fn do_x(&self) -> [u8; 3] {
        [0u8; 3]
    }
}

fn main() {}

