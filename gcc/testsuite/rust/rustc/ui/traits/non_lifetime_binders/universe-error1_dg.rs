#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Other<U: ?Sized> {}

impl<U: ?Sized> Other<U> for U {}

#[rustfmt::skip]
fn foo<U: ?Sized>()
where
    for<T> T: Other<U> {}

fn bar() {
    foo::<_>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

