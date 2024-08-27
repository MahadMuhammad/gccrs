#![feature(generic_const_items)]
#![allow(incomplete_features)]

// Check that we forbid elided lifetimes inside the generics of const items.

const K<T>: () = ()
where
    &T: Copy; // { dg-error ".E0637." "" { target *-*-* } }

const I<const S: &str>: &str = "";
// { dg-error ".E0637." "" { target *-*-* } .-1 }
// { dg-error ".E0637." "" { target *-*-* } .-2 }

const B<T: Trait<'_>>: () = (); // { dg-error ".E0637." "" { target *-*-* } }

trait Trait<'a> {}

fn main() {}

