#![feature(generic_const_items)]
#![allow(incomplete_features)]

trait Trait<P> {
    const A: ();
    const B<const K: u64, const Q: u64>: u64;
    const C<T>: T;
    const D<const N: usize>: usize;
    const E<'a>: &'a ();

    const F: usize;
    const G<T: PartialEq>: ();
}

impl<P> Trait<P> for () {
    const A<T>: () = ();
// { dg-error ".E0049." "" { target *-*-* } .-1 }
    const B<const K: u64>: u64 = 0;
// { dg-error ".E0049." "" { target *-*-* } .-1 }
    const C<'a>: &'a str = "";
// { dg-error ".E0049." "" { target *-*-* } .-1 }
    const D<const N: u16>: u16 = N;
// { dg-error ".E0053." "" { target *-*-* } .-1 }
    const E: &'static () = &();
// { dg-error ".E0195." "" { target *-*-* } .-1 }

    const F: usize = 1024
    where
        P: Copy; // { dg-error ".E0276." "" { target *-*-* } }
    const G<T: Eq>: () = (); // { dg-error ".E0276." "" { target *-*-* } }
}

fn main() {}

