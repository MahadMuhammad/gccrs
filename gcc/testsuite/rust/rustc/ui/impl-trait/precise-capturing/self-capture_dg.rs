trait Foo {
    fn bar<'a>() -> impl Sized + use<Self>;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

