#![crate_type="lib"]

struct S<N>;
// { dg-error ".E0091." "" { target *-*-* } .-1 }
// { help ".E0392." "" { target *-*-* } .-2 }
// { help ".E0392." "" { target *-*-* } .-3 }

// Ensure that we don't emit the const param suggestion here:
struct T<N: Copy>;
// { dg-error ".E0091." "" { target *-*-* } .-1 }
// { help ".E0392." "" { target *-*-* } .-2 }

type A<N> = ();
// { dg-error ".E0091." "" { target *-*-* } .-1 }
// { help ".E0091." "" { target *-*-* } .-2 }
// { help ".E0091." "" { target *-*-* } .-3 }

// Ensure that we don't emit the const param suggestion here:
type B<N: Copy> = ();
// { dg-error ".E0091." "" { target *-*-* } .-1 }
// { help ".E0091." "" { target *-*-* } .-2 }
type C<N: Sized> = ();
// { dg-error ".E0091." "" { target *-*-* } .-1 }
// { help ".E0091." "" { target *-*-* } .-2 }
type D<N: ?Sized> = ();
// { dg-error ".E0091." "" { target *-*-* } .-1 }
// { help ".E0091." "" { target *-*-* } .-2 }

