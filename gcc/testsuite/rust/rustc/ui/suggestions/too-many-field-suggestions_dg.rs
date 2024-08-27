struct Thing {
    a0: Foo,
    a1: Foo,
    a2: Foo,
    a3: Foo,
    a4: Foo,
    a5: Foo,
    a6: Foo,
    a7: Foo,
    a8: Foo,
    a9: Foo,
}

struct Foo {
    field: Field,
}

struct Field;

impl Foo {
    fn bar(&self) {}
}

fn bar(t: Thing) {
    t.bar(); // { dg-error ".E0599." "" { target *-*-* } }
    t.field; // { dg-error ".E0609." "" { target *-*-* } }
}

fn main() {}

