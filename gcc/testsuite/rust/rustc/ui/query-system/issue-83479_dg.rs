#![feature(type_alias_impl_trait)]

type PairCoupledTypes: Trait<
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }
    [u32; {
        static FOO: usize; // { dg-error "" "" { target *-*-* } }
    }],
> = impl Trait<
// { dg-error ".E0405." "" { target *-*-* } .-1 }
    [u32; {
        static FOO: usize; // { dg-error "" "" { target *-*-* } }
    }],
>;

fn main() {}

