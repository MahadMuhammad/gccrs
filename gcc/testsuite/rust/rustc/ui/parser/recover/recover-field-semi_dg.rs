struct Foo {
    foo: i32;
// { dg-error "" "" { target *-*-* } .-1 }
}

union Bar { // { dg-error "" "" { target *-*-* } }
    foo: i32;
// { dg-error "" "" { target *-*-* } .-1 }
}

enum Baz {
    Qux { foo: i32; }
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

