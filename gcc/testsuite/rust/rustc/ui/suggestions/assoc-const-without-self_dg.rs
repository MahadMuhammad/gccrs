struct Foo;

impl Foo {
    const A_CONST: usize = 1;

    fn foo() -> usize {
        A_CONST // { dg-error ".E0425." "" { target *-*-* } }
    }
}

fn main() {}

