#![crate_type = "lib"]

fn foo<T, U>(_: U) {}

fn bar() {
    foo(|| {}); // { dg-error ".E0282." "" { target *-*-* } }
}

