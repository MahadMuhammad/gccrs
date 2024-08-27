#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn foo<const N: usize>(
    _: [u8; {
        {
            N
        }
    }],
) {
}

fn ice<const L: usize>()
where
    [(); (L - 1) + 1 + L]:,
{
    foo::<_, L>([(); L + 1 + L]);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
// { dg-error ".E0308." "" { target *-*-* } .-4 }
// { dg-error ".E0308." "" { target *-*-* } .-5 }
// { dg-error ".E0308." "" { target *-*-* } .-6 }
}

fn main() {}

