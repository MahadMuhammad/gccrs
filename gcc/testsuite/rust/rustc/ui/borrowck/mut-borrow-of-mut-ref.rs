// Suggest not mutably borrowing a mutable reference
#![crate_type = "rlib"]

pub fn f(b: &mut i32) {
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-note ".E0596." "" { target *-*-* } .-2 }
// { dg-note ".E0596." "" { target *-*-* } .-3 }
    h(&mut b);
// { dg-note "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    g(&mut &mut b);
// { dg-note "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

pub fn g(b: &mut i32) { // { dg-note "" "" { target *-*-* } }
    h(&mut &mut b);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-note ".E0596." "" { target *-*-* } .-2 }
// { help ".E0596." "" { target *-*-* } .-3 }
}

pub fn h(_: &mut i32) {}

trait Foo {
    fn bar(&mut self);
}

impl Foo for &mut String {
    fn bar(&mut self) {}
}

pub fn baz(f: &mut String) { // { help "" "" { target *-*-* } }
    f.bar(); // { dg-error ".E0596." "" { target *-*-* } }
// { dg-note ".E0596." "" { target *-*-* } .-1 }
}

