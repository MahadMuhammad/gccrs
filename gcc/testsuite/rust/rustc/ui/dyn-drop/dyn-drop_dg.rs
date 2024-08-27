#![deny(dyn_drop)]
#![allow(bare_trait_objects)]
fn foo(_: Box<dyn Drop>) {} // { dg-error "" "" { target *-*-* } }
fn bar(_: &dyn Drop) {} // { dg-error "" "" { target *-*-* } }
fn baz(_: *mut Drop) {} // { dg-error "" "" { target *-*-* } }
struct Foo {
  _x: Box<dyn Drop> // { dg-error "" "" { target *-*-* } }
}
trait Bar {
  type T: ?Sized;
}
struct Baz {}
impl Bar for Baz {
  type T = dyn Drop; // { dg-error "" "" { target *-*-* } }
}
fn main() {}

