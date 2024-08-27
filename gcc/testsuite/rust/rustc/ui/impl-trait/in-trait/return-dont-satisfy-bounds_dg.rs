trait Foo<T> {
    fn foo<F2>(self) -> impl Foo<T>;
}

struct Bar;

impl Foo<char> for Bar {
    fn foo<F2: Foo<u8>>(self) -> impl Foo<u8> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
        self
    }
}

fn main() {}

