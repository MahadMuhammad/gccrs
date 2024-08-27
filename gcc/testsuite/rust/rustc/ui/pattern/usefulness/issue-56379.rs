enum Foo {
    A(bool),
    B(bool),
    C(bool),
}

fn main() {
    match Foo::A(true) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        Foo::A(true) => {}
        Foo::B(true) => {}
        Foo::C(true) => {}
    }
}

