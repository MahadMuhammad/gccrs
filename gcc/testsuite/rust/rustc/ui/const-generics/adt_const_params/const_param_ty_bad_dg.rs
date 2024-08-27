#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]

fn check(_: impl std::marker::UnsizedConstParamTy) {}

fn main() {
    check(main); // { dg-error ".E0277." "" { target *-*-* } }
    check(|| {}); // { dg-error ".E0277." "" { target *-*-* } }
    check(main as fn()); // { dg-error ".E0277." "" { target *-*-* } }
    check(&mut ()); // { dg-error ".E0277." "" { target *-*-* } }
    check(&mut () as *mut ()); // { dg-error ".E0277." "" { target *-*-* } }
    check(&() as *const ()); // { dg-error ".E0277." "" { target *-*-* } }
}

