#![crate_type = "lib"]
#![doc(as_ptr)]
// { dg-error "" "" { target *-*-* } .-1 }

#[doc(as_ptr)]
// { dg-error "" "" { target *-*-* } .-1 }
pub fn foo() {}

#[doc(123)]
// { dg-error "" "" { target *-*-* } .-1 }
#[doc("hello", "bar")]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
#[doc(foo::bar, crate::bar::baz = "bye")]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
fn bar() {}

