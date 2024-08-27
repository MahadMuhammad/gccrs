struct Foo {
    val: MissingType,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

fn main() {
    Foo { val: Default::default() };
}

