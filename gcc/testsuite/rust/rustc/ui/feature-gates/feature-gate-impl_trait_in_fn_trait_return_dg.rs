fn f() -> impl Fn() -> impl Sized { || () }
// { dg-error ".E0562." "" { target *-*-* } .-1 }
fn g() -> &'static dyn Fn() -> impl Sized { &|| () }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

fn main() {}

