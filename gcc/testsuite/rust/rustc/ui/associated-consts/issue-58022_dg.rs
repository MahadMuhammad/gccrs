pub trait Foo: Sized {
    const SIZE: usize;

    fn new(slice: &[u8; Foo::SIZE]) -> Self;
// { dg-error ".E0790." "" { target *-*-* } .-1 }
}

pub struct Bar<T: ?Sized>(T);

impl Bar<[u8]> {
    const SIZE: usize = 32;

    fn new(slice: &[u8; Self::SIZE]) -> Self {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        Foo(Box::new(*slice))
// { dg-error ".E0423." "" { target *-*-* } .-1 }
    }
}

fn main() {}

