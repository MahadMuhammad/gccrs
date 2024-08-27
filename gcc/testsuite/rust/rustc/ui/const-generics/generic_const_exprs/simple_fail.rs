#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

type Arr<const N: usize> = [u8; N - 1];
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn test<const N: usize>() -> Arr<N>
where
    [u8; N - 1]: Sized,
// { dg-error ".E0080." "" { target *-*-* } .-1 }
{
    todo!()
}

fn main() {
    test::<0>();
}

