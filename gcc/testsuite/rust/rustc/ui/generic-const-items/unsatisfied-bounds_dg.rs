#![feature(generic_const_items)]
#![allow(incomplete_features)]

// Ensure that we check if bounds on const items hold or not.

use std::convert::Infallible;

const C<T: Copy>: () = ();

const K<T>: () = ()
where
    Infallible: From<T>;

trait Trait<P> {
    const A: u32
    where
        P: Copy;

    const B<T>: u32
    where
        Infallible: From<T>;
}

impl<P> Trait<P> for () {
    const A: u32 = 0;
    const B<T>: u32 = 1;
}

fn main() {
    let () = C::<String>; // { dg-error ".E0277." "" { target *-*-* } }
    let () = K::<()>; // { dg-error ".E0277." "" { target *-*-* } }
    let _ = <() as Trait<Vec<u8>>>::A; // { dg-error ".E0277." "" { target *-*-* } }
    let _ = <() as Trait<&'static str>>::B::<()>; // { dg-error ".E0277." "" { target *-*-* } }
}

