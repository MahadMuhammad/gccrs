fn foo<T>() where T: for<'a> 'a {}
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }

fn main() {}

