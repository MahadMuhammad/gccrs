#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

pub fn foo()
where
    for<V> V: Sized,
{
}

pub fn bar()
where
    for<V> V: IntoIterator,
{
}

fn main() {
    foo();
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    bar();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

