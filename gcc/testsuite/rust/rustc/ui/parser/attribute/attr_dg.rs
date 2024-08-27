#![feature(lang_items)]

fn main() {}

#![lang = "foo"] // { dg-error "" "" { target *-*-* } }
fn foo() {}

