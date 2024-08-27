#![allow(non_camel_case_types)]

struct bool;

fn foo(_: bool) {}

fn main() {
    foo(true);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

