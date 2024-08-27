// Regression test for #127441

// Tests that we make the correct suggestion
// in case there are more than one `?Sized`
// bounds on a function parameter

use std::fmt::Debug;

fn foo1<T: ?Sized>(a: T) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn foo2<T: ?Sized + ?Sized>(a: T) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn foo3<T: ?Sized + ?Sized + Debug>(a: T) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn foo4<T: ?Sized + Debug + ?Sized >(a: T) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn foo5(_: impl ?Sized) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn foo6(_: impl ?Sized + ?Sized) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn foo7(_: impl ?Sized + ?Sized + Debug) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn foo8(_: impl ?Sized + Debug + ?Sized ) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn main() {}

