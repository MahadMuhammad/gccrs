#![allow(incomplete_features)]
#![feature(non_lifetime_binders)]

trait Foo: for<T> Bar<T> {}

trait Bar<T: ?Sized> {
    fn method(&self) {}
}

struct Type2;
fn needs_bar(_: *mut Type2) {}

fn main() {
    let x: &dyn Foo = &();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }

    needs_bar(x);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

