//@ aux-build:extern-issue-98562.rs

extern crate extern_issue_98562;
use extern_issue_98562::TraitA;

struct X;
impl TraitA<u8, u16, u32> for X {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
}
// { help "" "" { target *-*-* } .-1 }

fn main() {}

