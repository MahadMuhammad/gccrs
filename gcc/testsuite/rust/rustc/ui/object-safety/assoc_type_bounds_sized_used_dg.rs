//! This test checks that associated types with `Self: Sized` cannot be projected
//! from a `dyn Trait`.

trait Bop {
    type Bar: Default
    where
        Self: Sized;
}

fn bop<T: Bop + ?Sized>() {
    let _ = <T as Bop>::Bar::default();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

fn main() {
    bop::<dyn Bop>();
}

