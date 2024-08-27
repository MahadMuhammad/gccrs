// Regression test for #91450.
// This test ensures that the compiler does not suggest `Foo<[type error]>` in diagnostic messages.

fn foo() -> Option<_> {} // { dg-error ".E0121." "" { target *-*-* } }
// { dg-error ".E0121." "" { target *-*-* } .-1 }

fn main() {}

