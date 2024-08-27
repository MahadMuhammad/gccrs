#![crate_type = "lib"]

#[cfi_encoding = "3Bar"] // { dg-error ".E0658." "" { target *-*-* } }
pub struct Foo(i32);

