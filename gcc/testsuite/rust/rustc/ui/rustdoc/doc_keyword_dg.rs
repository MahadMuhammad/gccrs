#![crate_type = "lib"]
#![feature(rustdoc_internals)]

#![doc(keyword = "hello")] // { dg-error "" "" { target *-*-* } }

#[doc(keyword = "hell")] // { dg-error "" "" { target *-*-* } }
mod foo {
    fn hell() {}
}

#[doc(keyword = "hall")] // { dg-error "" "" { target *-*-* } }
fn foo() {}


// Regression test for the ICE described in #83512.
trait Foo {
    #[doc(keyword = "match")]
// { dg-error "" "" { target *-*-* } .-1 }
    fn quux() {}
}

