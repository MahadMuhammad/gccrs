//@ aux-build:missing-item-sugg.rs

extern crate missing_item_sugg;

struct Local;
impl missing_item_sugg::Foo for Local {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
}
// { help "" "" { target *-*-* } .-1 }

fn main() {}

