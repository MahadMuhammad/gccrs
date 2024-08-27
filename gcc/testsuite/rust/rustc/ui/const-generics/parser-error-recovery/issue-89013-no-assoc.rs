trait Foo<const N: usize> {
    fn do_x(&self) -> [u8; N];
}

struct Bar;

const T: usize = 42;

impl Foo<const 3> for Bar {
// { dg-error "" "" { target *-*-* } .-1 }
    fn do_x(&self) -> [u8; 3] {
        [0u8; 3]
    }
}

fn main() {}

