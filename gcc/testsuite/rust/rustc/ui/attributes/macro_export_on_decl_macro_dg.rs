// Using #[macro_export] on a decl macro has no effect and should warn

#![feature(decl_macro)]
#![deny(unused)]

#[macro_export] // { dg-error "" "" { target *-*-* } }
pub macro foo() {}

fn main() {}

