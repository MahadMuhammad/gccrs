enum Foo {
    enum Bar { Baz },
// { dg-error "" "" { target *-*-* } .-1 }
    struct Quux { field: u8 },
// { dg-error "" "" { target *-*-* } .-1 }
    union Wibble { field: u8 },
// { dg-error "" "" { target *-*-* } .-1 }
    Bat,
}

fn main() { }

