// Test that the `non_exhaustive_omitted_patterns` lint is triggered correctly.

#![feature(non_exhaustive_omitted_patterns_lint, unstable_test_feature)]
#![deny(unreachable_patterns)]

//@ aux-build:enums.rs
extern crate enums;
//@ aux-build:unstable.rs
extern crate unstable;
//@ aux-build:structs.rs
extern crate structs;

use enums::{
    EmptyNonExhaustiveEnum, NestedNonExhaustive, NonExhaustiveEnum, NonExhaustiveSingleVariant,
    VariantNonExhaustive,
};
use structs::{FunctionalRecord, MixedVisFields, NestedStruct, NormalStruct};
use unstable::{OnlyUnstableEnum, OnlyUnstableStruct, UnstableEnum, UnstableStruct};

#[non_exhaustive]
#[derive(Default)]
pub struct Foo {
    a: u8,
    b: usize,
    c: String,
}

#[non_exhaustive]
pub enum Bar {
    A,
    B,
    C,
}

fn no_lint() {
    let non_enum = NonExhaustiveEnum::Unit;
    // Ok: without the attribute
    match non_enum {
        NonExhaustiveEnum::Unit => {}
        NonExhaustiveEnum::Tuple(_) => {}
        _ => {}
    }
}

#[deny(non_exhaustive_omitted_patterns)]
fn main() {
    let enumeration = Bar::A;

    // Ok: this is a crate local non_exhaustive enum
    match enumeration {
        Bar::A => {}
        Bar::B => {}
        _ => {}
    }

    let non_enum = NonExhaustiveEnum::Unit;

    #[allow(non_exhaustive_omitted_patterns)]
    match non_enum {
        NonExhaustiveEnum::Unit => {}
        NonExhaustiveEnum::Tuple(_) => {}
        _ => {}
    }

    match non_enum {
// { dg-error "" "" { target *-*-* } .-1 }
        NonExhaustiveEnum::Unit => {}
        NonExhaustiveEnum::Tuple(_) => {}
        _ => {}
    }

    match non_enum {
// { dg-error "" "" { target *-*-* } .-1 }
        NonExhaustiveEnum::Unit | NonExhaustiveEnum::Struct { .. } => {}
        _ => {}
    }

    let x = 5;
    // We ignore the guard.
    match non_enum {
        NonExhaustiveEnum::Unit if x > 10 => {}
        NonExhaustiveEnum::Tuple(_) => {}
        NonExhaustiveEnum::Struct { .. } => {}
        _ => {}
    }

    match (non_enum, true) {
        (NonExhaustiveEnum::Unit, true) => {}
        (NonExhaustiveEnum::Tuple(_), false) => {}
        (NonExhaustiveEnum::Struct { .. }, false) => {}
        _ => {}
    }
    match (non_enum, true) {
// { dg-error "" "" { target *-*-* } .-1 }
        (NonExhaustiveEnum::Unit, true) => {}
        (NonExhaustiveEnum::Tuple(_), false) => {}
        _ => {}
    }

    match (true, non_enum) {
        (true, NonExhaustiveEnum::Unit) => {}
        (false, NonExhaustiveEnum::Tuple(_)) => {}
        (false, NonExhaustiveEnum::Struct { .. }) => {}
        _ => {}
    }
    match (true, non_enum) {
// { dg-error "" "" { target *-*-* } .-1 }
        (true, NonExhaustiveEnum::Unit) => {}
        (false, NonExhaustiveEnum::Tuple(_)) => {}
        _ => {}
    }

    match Some(non_enum) {
// { dg-error "" "" { target *-*-* } .-1 }
        Some(NonExhaustiveEnum::Unit) => {}
        Some(NonExhaustiveEnum::Tuple(_)) => {}
        _ => {}
    }

    // Ok: all covered and not `unreachable-patterns`
    #[deny(unreachable_patterns)]
    match non_enum {
        NonExhaustiveEnum::Unit => {}
        NonExhaustiveEnum::Tuple(_) => {}
        NonExhaustiveEnum::Struct { .. } => {}
        _ => {}
    }

    match NestedNonExhaustive::B {
// { dg-error "" "" { target *-*-* } .-1 }
        NestedNonExhaustive::A(NonExhaustiveEnum::Unit) => {}
        NestedNonExhaustive::A(_) => {}
        NestedNonExhaustive::B => {}
        _ => {}
    }

    match VariantNonExhaustive::Baz(1, 2) {
        VariantNonExhaustive::Baz(_, _) => {}
        VariantNonExhaustive::Bar { x, .. } => {}
    }
// { dg-error "" "" { target *-*-* } .-2 }

    let FunctionalRecord { first_field, second_field, .. } = FunctionalRecord::default();
// { dg-error "" "" { target *-*-* } .-1 }

    // Ok: this is local
    let Foo { a, b, .. } = Foo::default();

    let NestedStruct { bar: NormalStruct { first_field, .. }, .. } = NestedStruct::default();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    // Ok: this tests https://github.com/rust-lang/rust/issues/89382
    let MixedVisFields { a, b, .. } = MixedVisFields::default();

    // Ok: this only has 1 variant
    match NonExhaustiveSingleVariant::A(true) {
        NonExhaustiveSingleVariant::A(true) => {}
        _ => {}
    }

    // We can't catch the case below, so for consistency we don't catch this one either.
    match NonExhaustiveSingleVariant::A(true) {
        _ => {}
    }
    // We can't catch this case, because this would require digging fully through all the values of
    // any type we encounter. We need to be able to only consider present constructors.
    match &NonExhaustiveSingleVariant::A(true) {
        _ => {}
    }

    match Some(NonExhaustiveSingleVariant::A(true)) {
        Some(_) => {}
        None => {}
    }
    match Some(&NonExhaustiveSingleVariant::A(true)) {
        Some(_) => {}
        None => {}
    }

    // Ok: we don't lint on `if let` expressions
    if let NonExhaustiveEnum::Tuple(_) = non_enum {}

    match UnstableEnum::Stable {
// { dg-error "" "" { target *-*-* } .-1 }
        UnstableEnum::Stable => {}
        UnstableEnum::Stable2 => {}
        _ => {}
    }

    // Ok: the feature is on and all variants are matched
    match UnstableEnum::Stable {
        UnstableEnum::Stable => {}
        UnstableEnum::Stable2 => {}
        UnstableEnum::Unstable => {}
        _ => {}
    }

    // Ok: the feature is on and both variants are matched
    match OnlyUnstableEnum::Unstable {
        OnlyUnstableEnum::Unstable => {}
        OnlyUnstableEnum::Unstable2 => {}
        _ => {}
    }

    match OnlyUnstableEnum::Unstable {
// { dg-error "" "" { target *-*-* } .-1 }
        OnlyUnstableEnum::Unstable => {}
        _ => {}
    }

    let OnlyUnstableStruct { unstable, .. } = OnlyUnstableStruct::new();
// { dg-error "" "" { target *-*-* } .-1 }

    // OK: both unstable fields are matched with feature on
    let OnlyUnstableStruct { unstable, unstable2, .. } = OnlyUnstableStruct::new();

    let UnstableStruct { stable, stable2, .. } = UnstableStruct::default();
// { dg-error "" "" { target *-*-* } .-1 }

    // OK: both unstable and stable fields are matched with feature on
    let UnstableStruct { stable, stable2, unstable, .. } = UnstableStruct::default();

    // Ok: local bindings are allowed
    let local = NonExhaustiveEnum::Unit;

    // Ok: missing patterns will be blocked by the pattern being refutable
    let local_refutable @ NonExhaustiveEnum::Unit = NonExhaustiveEnum::Unit;
// { dg-error ".E0005." "" { target *-*-* } .-1 }

    // Check that matching on a reference results in a correct diagnostic
    match &non_enum {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        NonExhaustiveEnum::Unit => {}
        NonExhaustiveEnum::Tuple(_) => {}
        _ => {}
    }

    match (true, &non_enum) {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        (true, NonExhaustiveEnum::Unit) => {}
        _ => {}
    }

    match (&non_enum, true) {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        (NonExhaustiveEnum::Unit, true) => {}
        _ => {}
    }

    match Some(&non_enum) {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        Some(NonExhaustiveEnum::Unit | NonExhaustiveEnum::Tuple(_)) => {}
        _ => {}
    }
}

#[deny(non_exhaustive_omitted_patterns)]
// Ok: Pattern in a param is always wildcard
pub fn takes_non_exhaustive(_: NonExhaustiveEnum) {
    let _closure = |_: NonExhaustiveEnum| {};
}

// ICE #117033
enum Void {}
#[deny(non_exhaustive_omitted_patterns)]
pub fn void(v: Void) -> ! {
    match v {}
}

