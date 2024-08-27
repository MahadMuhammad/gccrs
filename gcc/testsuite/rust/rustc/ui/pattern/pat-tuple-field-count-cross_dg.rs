//@ aux-build:declarations-for-tuple-field-count-errors.rs

extern crate declarations_for_tuple_field_count_errors;

use declarations_for_tuple_field_count_errors::*;

fn main() {
    match Z0 {
        Z0() => {} // { dg-error ".E0532." "" { target *-*-* } }
        Z0(x) => {} // { dg-error ".E0532." "" { target *-*-* } }
    }
    match Z1() {
        Z1 => {} // { dg-error ".E0530." "" { target *-*-* } }
        Z1(x) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }

    match S(1, 2, 3) {
        S() => {} // { dg-error ".E0023." "" { target *-*-* } }
        S(1) => {} // { dg-error ".E0023." "" { target *-*-* } }
        S(xyz, abc) => {} // { dg-error ".E0023." "" { target *-*-* } }
        S(1, 2, 3, 4) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }
    match M(1, 2, 3) {
        M() => {} // { dg-error ".E0023." "" { target *-*-* } }
        M(1) => {} // { dg-error ".E0023." "" { target *-*-* } }
        M(xyz, abc) => {} // { dg-error ".E0023." "" { target *-*-* } }
        M(1, 2, 3, 4) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }

    match E1::Z0 {
        E1::Z0() => {} // { dg-error ".E0532." "" { target *-*-* } }
        E1::Z0(x) => {} // { dg-error ".E0532." "" { target *-*-* } }
    }
    match E1::Z1() {
        E1::Z1 => {} // { dg-error ".E0532." "" { target *-*-* } }
        E1::Z1(x) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }
    match E1::S(1, 2, 3) {
        E1::S() => {} // { dg-error ".E0023." "" { target *-*-* } }
        E1::S(1) => {} // { dg-error ".E0023." "" { target *-*-* } }
        E1::S(xyz, abc) => {} // { dg-error ".E0023." "" { target *-*-* } }
        E1::S(1, 2, 3, 4) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }

    match E2::S(1, 2, 3) {
        E2::S() => {} // { dg-error ".E0023." "" { target *-*-* } }
        E2::S(1) => {} // { dg-error ".E0023." "" { target *-*-* } }
        E2::S(xyz, abc) => {} // { dg-error ".E0023." "" { target *-*-* } }
        E2::S(1, 2, 3, 4) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }
    match E2::M(1, 2, 3) {
        E2::M() => {} // { dg-error ".E0023." "" { target *-*-* } }
        E2::M(1) => {} // { dg-error ".E0023." "" { target *-*-* } }
        E2::M(xyz, abc) => {} // { dg-error ".E0023." "" { target *-*-* } }
        E2::M(1, 2, 3, 4) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }
}

