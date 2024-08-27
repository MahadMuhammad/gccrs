fn lifetime_in_bounds<'a>(x: &'a ()) -> impl Into<&'a ()> + use<> { x }
// { dg-error "" "" { target *-*-* } .-1 }

fn lifetime_in_hidden<'a>(x: &'a ()) -> impl Sized + use<> { x }
// { dg-error ".E0700." "" { target *-*-* } .-1 }

fn main() {}

