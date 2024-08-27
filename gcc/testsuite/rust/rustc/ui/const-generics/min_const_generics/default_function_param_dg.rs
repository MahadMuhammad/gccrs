#![crate_type = "lib"]

fn foo<const SIZE: usize = 5usize>() {}
// { dg-error "" "" { target *-*-* } .-1 }

