trait A {
    type Type; // { dg-note "" "" { target *-*-* } }
    const CONST: usize = 1; // { dg-note "" "" { target *-*-* } }
    fn foo(&self); // { dg-note "" "" { target *-*-* } }
}

trait B {
    type Type; // { dg-note "" "" { target *-*-* } }
    const CONST: usize; // { dg-note "" "" { target *-*-* } }
    fn foo(&self); // { dg-note "" "" { target *-*-* } }
}

trait C: A + B {}

fn a<T: C>(t: T) {
    t.foo(); // { dg-error ".E0034." "" { target *-*-* } }
// { dg-note ".E0034." "" { target *-*-* } .-1 }
// { help ".E0034." "" { target *-*-* } .-2 }
// { help ".E0034." "" { target *-*-* } .-3 }
    let _ = T::CONST; // { dg-error ".E0034." "" { target *-*-* } }
// { dg-note ".E0034." "" { target *-*-* } .-1 }
// { help ".E0034." "" { target *-*-* } .-2 }
    let _: T::Type; // { dg-error ".E0223." "" { target *-*-* } }
// { dg-note ".E0221." "" { target *-*-* } .-1 }
// { help ".E0221." "" { target *-*-* } .-2 }
// { help ".E0221." "" { target *-*-* } .-3 }
}

#[derive(Debug)]
struct S;

impl<T: std::fmt::Debug> A for T {
    type Type = ();
    const CONST: usize = 1; // { dg-note "" "" { target *-*-* } }
    fn foo(&self) {} // { dg-note "" "" { target *-*-* } }
}

impl<T: std::fmt::Debug> B for T {
    type Type = ();
    const CONST: usize = 1; // { dg-note "" "" { target *-*-* } }
    fn foo(&self) {} // { dg-note "" "" { target *-*-* } }
}

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

