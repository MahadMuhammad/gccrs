// Regression test for issue #113378.
#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

#[const_trait]
trait Trait {
    const fn fun(); // { dg-error ".E0379." "" { target *-*-* } }
}

impl const Trait for () {
    const fn fun() {} // { dg-error ".E0379." "" { target *-*-* } }
}

impl Trait for u32 {
    const fn fun() {} // { dg-error ".E0379." "" { target *-*-* } }
}

trait NonConst {
    const fn fun(); // { dg-error ".E0379." "" { target *-*-* } }
}

fn main() {}

