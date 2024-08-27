//@ compile-flags: -Z parse-only
// { dg-additional-options "-frust-edition= 2021" }

struct S<
    T: 'a + Tr, // OK
    T: Tr + 'a, // OK
    T: 'a, // OK
    T:, // OK
    T: for<'a> ?Trait, // { dg-error "" "" { target *-*-* } }
    T: Tr +, // OK
    T: ?'a, // { dg-error "" "" { target *-*-* } }

    T: ~const Tr, // OK
    T: ~const ?Tr, // { dg-error "" "" { target *-*-* } }
    T: ~const Tr + 'a, // OK
    T: ~const 'a, // { dg-error "" "" { target *-*-* } }
    T: const 'a, // { dg-error "" "" { target *-*-* } }

    T: async Tr, // OK
    T: async ?Tr, // { dg-error "" "" { target *-*-* } }
    T: async Tr + 'a, // OK
    T: async 'a, // { dg-error "" "" { target *-*-* } }

    T: const async Tr, // OK
    T: const async ?Tr, // { dg-error "" "" { target *-*-* } }
    T: const async Tr + 'a, // OK
    T: const async 'a, // { dg-error "" "" { target *-*-* } }
>;

fn main() {}

