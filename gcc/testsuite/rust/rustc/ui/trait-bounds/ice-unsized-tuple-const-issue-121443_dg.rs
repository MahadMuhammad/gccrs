// Regression test for #121443
// Checks that no ICE occurs upon encountering
// a tuple with unsized element that is not
// the last element

type Fn = dyn FnOnce() -> u8;

const TEST: Fn = some_fn;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
const TEST2: (Fn, u8) = (TEST, 0);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

fn main() {}

