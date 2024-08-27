mod foo {
    enum Bar {
        Baz { a: isize },
    }
}

fn f(b: foo::Bar) {
// { dg-error ".E0603." "" { target *-*-* } .-1 }
    match b {
        foo::Bar::Baz { a: _a } => {} // { dg-error ".E0603." "" { target *-*-* } }
    }
}

fn main() {}

