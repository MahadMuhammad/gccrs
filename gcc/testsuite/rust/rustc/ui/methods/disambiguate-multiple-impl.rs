trait A {
    type Type;
    const CONST: usize;
    fn foo(&self);
}

trait B {
    type Type;
    const CONST: usize;
    fn foo(&self);
}

struct S;

impl A for S {
    type Type = ();
    const CONST: usize = 1; // { dg-note "" "" { target *-*-* } }
    fn foo(&self) {} // { dg-note "" "" { target *-*-* } }
}

impl B for S {
    type Type = ();
    const CONST: usize = 2; // { dg-note "" "" { target *-*-* } }
    fn foo(&self) {} // { dg-note "" "" { target *-*-* } }
}

fn main() {
    let s = S;
    S::foo(&s); // { dg-error ".E0034." "" { target *-*-* } }
// { dg-note ".E0034." "" { target *-*-* } .-1 }
// { help ".E0034." "" { target *-*-* } .-2 }
    let _: S::Type = (); // { dg-error ".E0223." "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
    let _ = S::CONST; // { dg-error ".E0034." "" { target *-*-* } }
// { dg-note ".E0034." "" { target *-*-* } .-1 }
// { help ".E0034." "" { target *-*-* } .-2 }
}

