// regression test for #98598

trait Foo {
    type Output;

    fn baz() -> Self::Output;
}

fn needs_infer<T>() {}

struct Bar {}

impl Foo for u8 {
    type Output = Bar;
    fn baz() -> Self::Output {
        needs_infer(); // { dg-error ".E0282." "" { target *-*-* } }
        Self::Output {}
    }
}

fn main() {}

