// Regression test for the ICE reported in issue #83471.

#![crate_type="lib"]
#![feature(no_core)]
#![no_core]

#[lang = "sized"]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
trait Sized {}

#[lang = "fn"]
// { dg-error ".E0718." "" { target *-*-* } .-1 }
// { dg-error ".E0718." "" { target *-*-* } .-2 }
trait Fn {
    fn call(export_name);
// { dg-error ".E0573." "" { target *-*-* } .-1 }
// { dg-warning ".E0573." "" { target *-*-* } .-2 }
// { dg-warning ".E0573." "" { target *-*-* } .-3 }
}
fn call_through_fn_trait() {
    a()
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

