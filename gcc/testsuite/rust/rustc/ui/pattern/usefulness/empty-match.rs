//@ revisions: normal exhaustive_patterns
//
// This tests a match with no arms on various types.
#![feature(never_type)]
#![cfg_attr(exhaustive_patterns, feature(exhaustive_patterns))]
#![deny(unreachable_patterns)]

fn nonempty<const N: usize>(arrayN_of_empty: [!; N]) {
    macro_rules! match_no_arms {
        ($e:expr) => {
            match $e {}
        };
    }
    macro_rules! match_guarded_arm {
        ($e:expr) => {
            match $e {
                _ if false => {}
            }
        };
    }

    struct NonEmptyStruct1;
    struct NonEmptyStruct2(bool);
    union NonEmptyUnion1 {
        foo: (),
    }
    union NonEmptyUnion2 {
        foo: (),
        bar: !,
    }
    enum NonEmptyEnum1 {
        Foo(bool),
    }
    enum NonEmptyEnum2 {
        Foo(bool),
        Bar,
    }
    enum NonEmptyEnum5 {
        V1,
        V2,
        V3,
        V4,
        V5,
    }
    let array0_of_empty: [!; 0] = [];

    match_no_arms!(0u8); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(0i8); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(0usize); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(0isize); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(NonEmptyStruct1); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(NonEmptyStruct2(true)); // { dg-error "" "" { target *-*-* } }
    match_no_arms!((NonEmptyUnion1 { foo: () })); // { dg-error "" "" { target *-*-* } }
    match_no_arms!((NonEmptyUnion2 { foo: () })); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(NonEmptyEnum1::Foo(true)); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(NonEmptyEnum2::Foo(true)); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(NonEmptyEnum5::V1); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(array0_of_empty); // { dg-error "" "" { target *-*-* } }
    match_no_arms!(arrayN_of_empty); // { dg-error "" "" { target *-*-* } }

    match_guarded_arm!(0u8); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(0i8); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(0usize); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(0isize); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(NonEmptyStruct1); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(NonEmptyStruct2(true)); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!((NonEmptyUnion1 { foo: () })); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!((NonEmptyUnion2 { foo: () })); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(NonEmptyEnum1::Foo(true)); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(NonEmptyEnum2::Foo(true)); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(NonEmptyEnum5::V1); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(array0_of_empty); // { dg-error "" "" { target *-*-* } }
    match_guarded_arm!(arrayN_of_empty); // { dg-error "" "" { target *-*-* } }
}

fn main() {}

