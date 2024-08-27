struct Bar(pub(()));

struct Foo {
    pub(crate) () foo: usize, // { dg-error "" "" { target *-*-* } }
}

fn main() {}

