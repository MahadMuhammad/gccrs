// Regression test for #85794

struct Baz {
    inner : dyn fn ()
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
// { dg-error ".E0412." "" { target *-*-* } .-3 }
}

fn main() {}

