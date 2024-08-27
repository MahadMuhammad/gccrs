#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn foo() where for<T> T: Copy {}

fn main() {
    foo();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

