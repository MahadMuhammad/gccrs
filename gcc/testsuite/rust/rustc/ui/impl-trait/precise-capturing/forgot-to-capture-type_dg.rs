fn type_param<T>() -> impl Sized + use<> {}
// { dg-error "" "" { target *-*-* } .-1 }

trait Foo {
    fn bar() -> impl Sized + use<>;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

