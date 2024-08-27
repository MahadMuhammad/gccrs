//@ aux-build: issue-91800-macro.rs

#[macro_use]
extern crate issue_91800_macro;

#[derive(MyTrait)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
#[attribute_macro]
// { dg-error "" "" { target *-*-* } .-1 }
struct MyStruct;

fn_macro! {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

