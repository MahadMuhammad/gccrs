#![feature(associated_type_defaults)]

trait A {
    type Type = ();
    const CONST: usize = 1; // { dg-note "" "" { target *-*-* } }
    fn foo(&self) {} // { dg-note "" "" { target *-*-* } }
}

trait B {
    type Type = ();
    const CONST: usize = 2; // { dg-note "" "" { target *-*-* } }
    fn foo(&self) {} // { dg-note "" "" { target *-*-* } }
}

#[derive(Debug)]
struct S;

impl<T: std::fmt::Debug> A for T {}

impl<T: std::fmt::Debug> B for T {}

fn main() {
    let s = S;
    S::foo(&s); // { dg-error ".E0034." "" { target *-*-* } }
// { dg-note ".E0034." "" { target *-*-* } .-1 }
// { help ".E0034." "" { target *-*-* } .-2 }
    let _ = S::CONST; // { dg-error ".E0034." "" { target *-*-* } }
// { dg-note ".E0034." "" { target *-*-* } .-1 }
// { help ".E0034." "" { target *-*-* } .-2 }
    let _: S::Type; // { dg-error ".E0223." "" { target *-*-* } }
// { help ".E0223." "" { target *-*-* } .-1 }
}

