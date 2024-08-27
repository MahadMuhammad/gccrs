trait Foo {
    type Output;

    fn baz() -> Self::Output;
}

fn needs_infer<T>() {}

enum Bar {
    Variant {}
}

impl Foo for u8 {
    type Output = Bar;
    fn baz() -> Self::Output {
        needs_infer(); // { dg-error ".E0282." "" { target *-*-* } }
        Self::Output::Variant {}
    }
}

fn main() {}

