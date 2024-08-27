#![feature(type_alias_impl_trait)]

mod case1 {
    type Opaque<'x> = impl Sized + 'x;
    fn foo<'s>() -> Opaque<'s> {
        let _ = || { let _: Opaque<'s> = (); };
// { dg-error ".E0792." "" { target *-*-* } .-1 }
    }
}

mod case2 {
    type Opaque<'x> = impl Sized + 'x;
    fn foo<'s>() -> Opaque<'s> {
        let _ = || -> Opaque<'s> {};
// { dg-error ".E0792." "" { target *-*-* } .-1 }
    }
}

fn main() {}

