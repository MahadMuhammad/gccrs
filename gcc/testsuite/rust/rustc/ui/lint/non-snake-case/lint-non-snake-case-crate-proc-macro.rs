//@ only-x86_64-unknown-linux-gnu
#![crate_type = "proc-macro"]
#![crate_name = "NonSnakeCase"]
// { dg-error "" "" { target *-*-* } .-1 }
#![deny(non_snake_case)]

fn main() {}

