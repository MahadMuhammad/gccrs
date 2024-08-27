// { dg-additional-options "-frust-edition=2021" }

#![feature(anonymous_lifetime_in_impl_trait)]

// This is understood as `fn foo<'_1>(_: impl Iterator<Item = &'_1 ()>) {}`.
fn f(_: impl Iterator<Item = &'_ ()>) {}

// But that lifetime does not participate in resolution.
fn g(mut x: impl Iterator<Item = &'_ ()>) -> Option<&'_ ()> { x.next() }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }

// This is understood as `fn foo<'_1>(_: impl Iterator<Item = &'_1 ()>) {}`.
async fn h(_: impl Iterator<Item = &'_ ()>) {}

// But that lifetime does not participate in resolution.
async fn i(mut x: impl Iterator<Item = &'_ ()>) -> Option<&'_ ()> { x.next() }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }

fn main() {}

