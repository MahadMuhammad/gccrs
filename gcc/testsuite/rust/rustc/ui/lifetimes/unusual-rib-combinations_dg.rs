struct S<'a>(&'a u8);
fn foo() {}

// Paren generic args in AnonConst
fn a() -> [u8; foo()] {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    panic!()
}

// Paren generic args in ConstGeneric
fn b<const C: u8()>() {}
// { dg-error ".E0214." "" { target *-*-* } .-1 }

// Paren generic args in AnonymousReportError
fn c<T = u8()>() {}
// { dg-error ".E0214." "" { target *-*-* } .-1 }
// { dg-error ".E0214." "" { target *-*-* } .-2 }
// { dg-warning ".E0214." "" { target *-*-* } .-3 }

// Elided lifetime in path in ConstGeneric
fn d<const C: S>() {}
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }

trait Foo<'a> {}
struct Bar<const N: &'a (dyn for<'a> Foo<'a>)>;
// { dg-error ".E0770." "" { target *-*-* } .-1 }
// { dg-error ".E0770." "" { target *-*-* } .-2 }

fn main() {}

