//@ run-rustfix
#![allow(dead_code)]
trait TraitWithAType {
    type Item;
}
trait Trait {}
struct A {}
impl TraitWithAType for A {
    type Item = dyn Trait; // { dg-error ".E0277." "" { target *-*-* } }
}
fn main() {}

