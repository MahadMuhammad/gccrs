// Check that we reject bivariant generic parameters as unused.
// Furthermore, check that we only emit a single diagnostic for unused type parameters:
// Previously, we would emit *two* errors, namely E0392 and E0091.

#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type A<'a> = ();
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { help ".E0392." "" { target *-*-* } .-2 }

type B<T> = ();
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { help ".E0392." "" { target *-*-* } .-2 }
// { help ".E0392." "" { target *-*-* } .-3 }

// Check that we don't emit the const param help message here:
type C<T: Copy> = ();
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { help ".E0392." "" { target *-*-* } .-2 }

fn main() {}

