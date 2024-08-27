//@ check-fail

static STATIC_VAR_FIVE: &One();
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }

fn main() {}

