#![feature(type_alias_impl_trait)]

mod lifetime_params {
    type Ty<'a> = impl Sized;
    fn defining(s: &str) -> Ty<'_> { s }
    fn execute(ty: Ty<'_>) -> &str { todo!() }
// { dg-error ".E0581." "" { target *-*-* } .-1 }
// { dg-error ".E0581." "" { target *-*-* } .-2 }

    type BadFnSig = fn(Ty<'_>) -> &str;
// { dg-error ".E0581." "" { target *-*-* } .-1 }
    type BadTraitRef = dyn Fn(Ty<'_>) -> &str;
// { dg-error ".E0582." "" { target *-*-* } .-1 }
}

mod lifetime_params_2 {
    type Ty<'a> = impl FnOnce() -> &'a str;
    fn defining(s: &str) -> Ty<'_> { move || s }
    fn execute(ty: Ty<'_>) -> &str { ty() }
// { dg-error ".E0581." "" { target *-*-* } .-1 }
// { dg-error ".E0581." "" { target *-*-* } .-2 }
}

// regression test for https://github.com/rust-lang/rust/issues/97104
mod type_params {
    type Ty<T> = impl Sized;
    fn define<T>(s: T) -> Ty<T> { s }

    type BadFnSig = fn(Ty<&str>) -> &str;
// { dg-error ".E0581." "" { target *-*-* } .-1 }
    type BadTraitRef = dyn Fn(Ty<&str>) -> &str;
// { dg-error ".E0582." "" { target *-*-* } .-1 }
}

fn main() {}

