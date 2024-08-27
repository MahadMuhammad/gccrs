//! This test checks that opaque types get unsized instead of
//! constraining their hidden type to a trait object.

//@ revisions: next old
//@[next] compile-flags: -Znext-solver

trait Trait {}

impl Trait for u32 {}

fn hello() -> Box<impl Trait + ?Sized> {
    if true {
        let x = hello();
// { dg-error "" "" { target *-*-* } .-1 }
        let y: Box<dyn Send> = x;
// { dg-error "" "" { target *-*-* } .-1 }
    }
    Box::new(1u32)
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

