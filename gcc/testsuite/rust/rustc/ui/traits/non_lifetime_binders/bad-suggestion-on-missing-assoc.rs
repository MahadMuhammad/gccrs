#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }
#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

// Test for <https://github.com/rust-lang/rust/issues/115497>,
// which originally relied on associated_type_bounds, but was
// minimized away from that.

trait TraitA {
    type AsA;
}
trait TraitB {
    type AsB;
}
trait TraitC {}

fn foo<T>()
where
    for<const N: u8 = { T::A }> T: TraitA<AsA = impl TraitB<AsB = impl TraitC>>,
// { dg-error ".E0562." "" { target *-*-* } .-1 }
// { dg-error ".E0562." "" { target *-*-* } .-2 }
// { dg-error ".E0562." "" { target *-*-* } .-3 }
{
}

fn main() {}

