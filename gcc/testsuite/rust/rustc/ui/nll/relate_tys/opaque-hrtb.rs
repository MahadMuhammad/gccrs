trait MyTrait<T> {}

struct Foo;
impl<T> MyTrait<T> for Foo {}

fn bar<Input>() -> impl MyTrait<Input> {
    Foo
}

fn foo() -> impl for<'a> MyTrait<&'a str> {
    bar() // { dg-error "" "" { target *-*-* } }
}

fn main() {}

