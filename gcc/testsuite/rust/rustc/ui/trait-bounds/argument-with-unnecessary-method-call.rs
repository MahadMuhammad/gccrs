struct Foo;
struct Bar;
impl From<Bar> for Foo {
    fn from(_: Bar) -> Self { Foo }
}
fn qux(_: impl From<Bar>) {}
fn main() {
    qux(Bar.into()); // { dg-error ".E0283." "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

