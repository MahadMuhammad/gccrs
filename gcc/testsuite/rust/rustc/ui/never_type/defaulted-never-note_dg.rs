//@ revisions: nofallback fallback
//@[nofallback] run-pass
//@[fallback] check-fail

// We need to opt into the `never_type_fallback` feature
// to trigger the requirement that this is testing.
#![cfg_attr(fallback, feature(never_type, never_type_fallback))]

#![allow(unused)]

trait Deserialize: Sized {
    fn deserialize() -> Result<Self, String>;
}

impl Deserialize for () {
    fn deserialize() -> Result<(), String> {
        Ok(())
    }
}

trait ImplementedForUnitButNotNever {}

impl ImplementedForUnitButNotNever for () {}

fn foo<T: ImplementedForUnitButNotNever>(_t: T) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
fn smeg() {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    let _x = return;
    foo(_x);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }
}

fn main() {
    smeg();
}

