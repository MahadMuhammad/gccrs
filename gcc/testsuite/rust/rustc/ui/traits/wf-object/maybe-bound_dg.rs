// Test that `dyn ... + ?Sized + ...` is okay (though `?Sized` has no effect in trait objects).

trait Foo {}

type _0 = dyn ?Sized + Foo;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

type _1 = dyn Foo + ?Sized;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

type _2 = dyn Foo + ?Sized + ?Sized;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

type _3 = dyn ?Sized + Foo;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

