trait Foo {
    type Output<T>;

    fn baz();
}

enum Bar<T> {
    Simple {},
    Generic(T),
}

impl Foo for u8 {
    type Output<T> = Bar<T>;
    fn baz() {
        Self::Output::Simple {}; // { dg-error ".E0282." "" { target *-*-* } }
    }
}

fn main() {}

