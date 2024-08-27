#![feature(custom_inner_attributes)]
#![rustfmt::skip]
// Test the "defined here" and "not covered" diagnostic hints.
// We also make sure that references are peeled off from the scrutinee type
// so that the diagnostics work better with default binding modes.

#[derive(Clone)]
enum E {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { dg-note "" "" { target *-*-* } .-7 }
// { dg-note "" "" { target *-*-* } .-8 }
// { dg-note "" "" { target *-*-* } .-9 }
    A,
    B,
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
    C
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-error "" "" { target *-*-* } .-6 }
}

fn by_val(e: E) {
    let e1 = e.clone();
    match e1 { // { dg-error ".E0004." "" { target *-*-* } }
// { dg-note ".E0004." "" { target *-*-* } .-1 }
// { dg-note ".E0004." "" { target *-*-* } .-2 }
        E::A => {}
    }

    let E::A = e;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
// { dg-error ".E0005." "" { target *-*-* } .-2 }
// { dg-note ".E0005." "" { target *-*-* } .-3 }
// { dg-note ".E0005." "" { target *-*-* } .-4 }
// { dg-note ".E0005." "" { target *-*-* } .-5 }
}

fn by_ref_once(e: &E) {
    match e {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-error ".E0004." "" { target *-*-* } .-2 }
// { dg-note ".E0004." "" { target *-*-* } .-3 }
        E::A => {}
    }

    let E::A = e;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
// { dg-error ".E0005." "" { target *-*-* } .-2 }
// { dg-note ".E0005." "" { target *-*-* } .-3 }
// { dg-note ".E0005." "" { target *-*-* } .-4 }
// { dg-note ".E0005." "" { target *-*-* } .-5 }
}

fn by_ref_thrice(e: & &mut &E) {
    match e {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-error ".E0004." "" { target *-*-* } .-2 }
// { dg-note ".E0004." "" { target *-*-* } .-3 }
        E::A => {}
    }

    let E::A = e;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
// { dg-error ".E0005." "" { target *-*-* } .-2 }
// { dg-note ".E0005." "" { target *-*-* } .-3 }
// { dg-note ".E0005." "" { target *-*-* } .-4 }
// { dg-note ".E0005." "" { target *-*-* } .-5 }
}

enum Opt {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    Some(u8),
    None,
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
}

fn ref_pat(e: Opt) {
    match e {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-error ".E0004." "" { target *-*-* } .-2 }
// { dg-note ".E0004." "" { target *-*-* } .-3 }
        Opt::Some(ref _x) => {}
    }

    let Opt::Some(ref _x) = e;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
// { dg-note ".E0005." "" { target *-*-* } .-2 }
// { dg-note ".E0005." "" { target *-*-* } .-3 }
// { dg-note ".E0005." "" { target *-*-* } .-4 }
// { dg-note ".E0005." "" { target *-*-* } .-5 }
}

fn main() {}

