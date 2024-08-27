trait Foo {
    type Bar;
}

impl Foo for () {
    type Bar = impl std::fmt::Debug;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
}

struct Mop;

impl Mop {
    type Bop = impl std::fmt::Debug;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
}

fn main() {}

