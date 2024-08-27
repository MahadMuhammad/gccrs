trait Foo {
    type Assoc<'a, const N: usize>;
}

fn foo<T: Foo>() {
    let _: <T as Foo>::Assoc<3>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

fn main() {}

