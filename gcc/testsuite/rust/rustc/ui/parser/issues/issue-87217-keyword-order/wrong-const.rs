// { dg-additional-options "-frust-edition=2018" }

// There is an order to respect for keywords before a function:
// `<visibility>, const, async, unsafe, extern, "<ABI>"`
//
// This test ensures the compiler is helpful about them being misplaced.
// Visibilities are tested elsewhere.

unsafe const fn test() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }

fn main() {}

