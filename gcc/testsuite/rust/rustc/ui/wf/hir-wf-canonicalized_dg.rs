//@ incremental

trait Foo {
    type V;
}

trait Callback<T: Foo>: Fn(&Bar<'_, T>, &T::V) {}

struct Bar<'a, T> {
    callback: Box<dyn Callback<dyn Callback<Bar<'a, T>>>>,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
}

impl<T: Foo> Bar<'_, Bar<'_, T>> {}

fn main() {}

