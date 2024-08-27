//@ revisions: normal exhaustive_patterns
#![cfg_attr(exhaustive_patterns, feature(exhaustive_patterns))]
#![feature(never_type)]

mod foo {
    pub struct SecretlyEmpty {
        _priv: !,
    }

    pub struct NotSoSecretlyEmpty {
        pub _pub: !,
    }
}

struct NotSoSecretlyEmpty {
    _priv: !,
}

enum Foo {
// { dg-note "" "" { target *-*-* } .-1 }
    A(foo::SecretlyEmpty),
// { dg-note "" "" { target *-*-* } .-1 }
    B(foo::NotSoSecretlyEmpty),
    C(NotSoSecretlyEmpty),
    D(u32, u32),
}

fn main() {
    let x: Foo = Foo::D(123, 456);
    let Foo::D(_y, _z) = x;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { help "" "" { target *-*-* } .-7 }
}

