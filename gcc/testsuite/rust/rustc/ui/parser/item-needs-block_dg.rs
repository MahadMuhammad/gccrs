trait Trait;
// { dg-error "" "" { target *-*-* } .-1 }

impl Trait for ();
// { dg-error "" "" { target *-*-* } .-1 }

enum Enum;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

