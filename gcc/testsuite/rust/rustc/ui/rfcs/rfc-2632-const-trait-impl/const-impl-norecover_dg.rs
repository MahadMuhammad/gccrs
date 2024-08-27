#![feature(const_trait_impl)]

struct Foo;

const impl Foo { // { dg-error "" "" { target *-*-* } }
    fn bar() {}
}

fn main() {
     // shouldn't error here because we shouldn't have been able to recover above
     Foo::bar();
}

