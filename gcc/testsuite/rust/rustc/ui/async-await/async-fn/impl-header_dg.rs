// { dg-additional-options "-frust-edition=2018" }

struct F;

impl async Fn<()> for F {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
// { dg-error ".E0046." "" { target *-*-* } .-3 }
// { dg-error ".E0046." "" { target *-*-* } .-4 }
// { dg-error ".E0046." "" { target *-*-* } .-5 }

fn main() {}

