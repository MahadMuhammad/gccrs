// Check for recursion involving references to trait-associated const default.

trait Foo {
    const BAR: u32;
}

trait FooDefault {
    const BAR: u32 = DEFAULT_REF_BAR; // { dg-error ".E0391." "" { target *-*-* } }
}

const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR;

struct GlobalDefaultRef;

impl FooDefault for GlobalDefaultRef {}

fn main() {}

