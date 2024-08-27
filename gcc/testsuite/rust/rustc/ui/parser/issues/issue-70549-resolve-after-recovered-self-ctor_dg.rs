struct S {}

impl S {
    fn foo(&mur Self) {}
// { dg-error ".E0533." "" { target *-*-* } .-1 }
// { dg-error ".E0533." "" { target *-*-* } .-2 }
// { dg-error ".E0533." "" { target *-*-* } .-3 }
    fn bar(&'static mur Self) {}
// { dg-error ".E0533." "" { target *-*-* } .-1 }
// { dg-error ".E0533." "" { target *-*-* } .-2 }
// { dg-error ".E0533." "" { target *-*-* } .-3 }
// { dg-error ".E0533." "" { target *-*-* } .-4 }

    fn baz(&mur Self @ _) {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

