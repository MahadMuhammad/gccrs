//@ check-fail

// Fine, but lints as unused
type Foo where u32: Copy = ();
// Not fine.
type Bar = () where u32: Copy;
// { dg-error "" "" { target *-*-* } .-1 }
type Baz = () where;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

