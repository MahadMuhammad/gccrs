struct Foo {
    v: u8,
    w: u8,
}
fn main() {
    builtin # offset_of(Foo, v); // { dg-error ".E0658." "" { target *-*-* } }
}

