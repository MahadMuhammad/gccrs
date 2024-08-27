#![feature(const_trait_impl)]

#[const_trait]
trait Trait {}

// Regression test for issue #90052.
fn non_const_function<T: ~const Trait>() {} // { dg-error "" "" { target *-*-* } }

struct Struct<T: ~const Trait> { field: T } // { dg-error "" "" { target *-*-* } }
struct TupleStruct<T: ~const Trait>(T); // { dg-error "" "" { target *-*-* } }
struct UnitStruct<T: ~const Trait>; // { dg-error ".E0392." "" { target *-*-* } }
// { dg-error ".E0392." "" { target *-*-* } .-1 }

enum Enum<T: ~const Trait> { Variant(T) } // { dg-error "" "" { target *-*-* } }

union Union<T: ~const Trait> { field: T } // { dg-error ".E0740." "" { target *-*-* } }
// { dg-error ".E0740." "" { target *-*-* } .-1 }

type Type<T: ~const Trait> = T; // { dg-error "" "" { target *-*-* } }

const CONSTANT<T: ~const Trait>: () = (); // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }

trait NonConstTrait {
    type Type<T: ~const Trait>: ~const Trait;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    fn non_const_function<T: ~const Trait>(); // { dg-error "" "" { target *-*-* } }
    const CONSTANT<T: ~const Trait>: (); // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

impl NonConstTrait for () {
    type Type<T: ~const Trait> = (); // { dg-error ".E0275." "" { target *-*-* } }
// { dg-error ".E0275." "" { target *-*-* } .-1 }
    fn non_const_function<T: ~const Trait>() {} // { dg-error "" "" { target *-*-* } }
    const CONSTANT<T: ~const Trait>: () = (); // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

struct Implementor;

impl Implementor {
    type Type<T: ~const Trait> = (); // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    fn non_const_function<T: ~const Trait>() {} // { dg-error "" "" { target *-*-* } }
    const CONSTANT<T: ~const Trait>: () = (); // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

// non-const traits
trait Child0: ~const Trait {} // { dg-error "" "" { target *-*-* } }
trait Child1 where Self: ~const Trait {} // { dg-error "" "" { target *-*-* } }

// non-const impl
impl<T: ~const Trait> Trait for T {} // { dg-error "" "" { target *-*-* } }

// inherent impl (regression test for issue #117004)
impl<T: ~const Trait> Struct<T> {} // { dg-error "" "" { target *-*-* } }

fn main() {}

