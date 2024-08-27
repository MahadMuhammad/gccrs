#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Foo: for<T> Bar<T> {}

trait Bar<T: ?Sized> {
    fn method(&self) {}
}

fn needs_bar(x: &(impl Bar<i32> + ?Sized)) {
    x.method();
}

impl Foo for () {}

impl<T: ?Sized> Bar<T> for () {}

fn main() {
    let x: &dyn Foo = &();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
    needs_bar(x);
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

