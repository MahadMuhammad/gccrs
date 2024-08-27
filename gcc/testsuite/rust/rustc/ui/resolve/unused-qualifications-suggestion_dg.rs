//@ run-rustfix

#![deny(unused_qualifications)]

mod foo {
    pub fn bar() {}
}

mod baz {
    pub mod qux {
        pub fn quux() {}
    }
}

fn main() {
    use foo::bar;
    foo::bar();
// { dg-error "" "" { target *-*-* } .-1 }
    bar();

    use baz::qux::quux;
    baz::qux::quux();
// { dg-error "" "" { target *-*-* } .-1 }
    quux();
}

