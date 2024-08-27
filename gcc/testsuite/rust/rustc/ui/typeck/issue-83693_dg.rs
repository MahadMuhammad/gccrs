// Regression test for the ICE described in #83693.

#![feature(fn_traits)]
#![crate_type="lib"]

impl F {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    fn call() {
        <Self as Fn(&TestResult)>::call
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
    }
}

fn call() {
    <x as Fn(&usize)>::call
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
}

