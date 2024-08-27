// { dg-additional-options "-frust-edition=2015" }
//@ check-pass
// issue: 114664

fn ice() -> impl AsRef<Fn(&())> {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
    Foo
}

struct Foo;
impl AsRef<dyn Fn(&())> for Foo {
    fn as_ref(&self) -> &(dyn for<'a> Fn(&'a ()) + 'static) {
        todo!()
    }
}

pub fn main() {}

