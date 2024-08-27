#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Trait {}
fn needs_trait(_: impl Trait) {}

trait Assoc {
    fn method() -> impl Sized;
    fn method_with_lt() -> impl Sized;
    fn method_with_ty<T>() -> impl Sized;
    fn method_with_ct<const N: usize>() -> impl Sized;
}

fn foo<T: Assoc>(t: T) {
    needs_trait(T::method());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    needs_trait(T::method_with_lt());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    needs_trait(T::method_with_ty());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    needs_trait(T::method_with_ct());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

