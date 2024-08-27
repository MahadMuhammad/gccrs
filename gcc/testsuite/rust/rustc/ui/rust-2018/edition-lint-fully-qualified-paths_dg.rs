//@ run-rustfix

#![deny(absolute_paths_not_starting_with_crate)]

mod foo {
    pub(crate) trait Foo {
        type Bar;
    }

    pub(crate) struct Baz {}

    impl Foo for Baz {
        type Bar = ();
    }
}

fn main() {
    let _: <foo::Baz as ::foo::Foo>::Bar = ();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }

    let _: <::foo::Baz as foo::Foo>::Bar = ();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

