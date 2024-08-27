// Tests that lint levels can be set for late lints.
#![allow(
    non_snake_case,
    overflowing_literals,
    missing_docs,
    dyn_drop,
    enum_intrinsics_non_enums,
    clashing_extern_declarations
)]

extern crate core;
use core::mem::{Discriminant, discriminant};

// The following is a check of the lints used here to verify they do not warn
// when allowed.
pub fn missing_docs_allowed() {} // missing_docs
fn dyn_drop_allowed(_x: Box<dyn Drop>) {} // dyn_drop
fn verify_no_warnings() {
    discriminant::<i32>(&123); // enum_intrinsics_non_enums
    let x: u8 = 1000; // overflowing_literals
    let NON_SNAKE_CASE = 1; // non_snake_case
}
mod clashing_extern_allowed {
    extern "C" {
        fn extern_allowed();
    }
}
extern "C" {
    fn extern_allowed(_: i32); // clashing_extern_declarations
}

// ################## Types

#[deny(missing_docs)]
pub type MissingDocType = i32; // { dg-error "" "" { target *-*-* } }

// There aren't any late lints that I can find that can be easily used with types.
// type BareFnPtr = fn(#[deny()]i32);
// type BareFnPtrVariadic = extern "C" fn(i32, #[deny()]...);

// ################## Items
#[deny(missing_docs)]
pub struct ItemOuter; // { dg-error "" "" { target *-*-* } }

pub mod module_inner { // { dg-error "" "" { target *-*-* } }
    #![deny(missing_docs)]
    pub fn missing_inner() {} // { dg-error "" "" { target *-*-* } }
}

pub struct Associated;
impl Associated {
    #![deny(missing_docs)]

    pub fn inherent_denied_from_inner() {} // { dg-error "" "" { target *-*-* } }
}

impl Associated {
    #[deny(missing_docs)]
    pub fn inherent_fn() {} // { dg-error "" "" { target *-*-* } }

    #[deny(missing_docs)]
    pub const INHERENT_CONST: i32 = 1; // { dg-error "" "" { target *-*-* } }
}

pub trait TraitInner { // { dg-error "" "" { target *-*-* } }
    #![deny(missing_docs)]
}

pub trait AssociatedTraitInner { // { dg-error "" "" { target *-*-* } }
    #![deny(missing_docs)]

    fn denied_from_inner() {} // { dg-error "" "" { target *-*-* } }
}

pub trait AssociatedTrait {
    fn denied_from_inner(_x: Box<dyn Drop>) {} // Used below

    #[deny(missing_docs)]
    fn assoc_fn() {} // { dg-error "" "" { target *-*-* } }

    #[deny(missing_docs)]
    const ASSOC_CONST: u8 = 1; // { dg-error "" "" { target *-*-* } }

    #[deny(missing_docs)]
    type AssocType; // { dg-error "" "" { target *-*-* } }
}

struct Foo;

impl AssociatedTrait for Associated {
    #![deny(dyn_drop)]

    fn denied_from_inner(_x: Box<dyn Drop>) {} // { dg-error "" "" { target *-*-* } }

    #[deny(enum_intrinsics_non_enums)]
    fn assoc_fn() { discriminant::<i32>(&123); } // { dg-error "" "" { target *-*-* } }

    #[deny(overflowing_literals)] const ASSOC_CONST: u8 = 1000; // { dg-error "" "" { target *-*-* } }
    type AssocType = i32;
}


// There aren't any late lints that can apply to a field that I can find.
// non_snake_case doesn't work on fields
// struct StructFields {
//     #[deny()]f1: i32,
// }
// struct StructTuple(#[deny()]i32);

pub enum Enum {
    #[deny(missing_docs)]
    Variant1, // { dg-error "" "" { target *-*-* } }
}

mod clashing_extern {
    extern "C" {
        fn clashing1();
        fn clashing2();
    }
}
extern "C" {
    #![deny(clashing_extern_declarations)]
    fn clashing1(_: i32); // { dg-error "" "" { target *-*-* } }
}

extern "C" {
    #[deny(clashing_extern_declarations)]
    fn clashing2(_: i32); // { dg-error "" "" { target *-*-* } }
}

fn function(#[deny(non_snake_case)] PARAM: i32) {} // { dg-error "" "" { target *-*-* } }
// There aren't any late lints that can apply to generics that I can find.
// fn generics<#[deny()]T>() {}


// ################## Statements
fn statements() {
    #[deny(enum_intrinsics_non_enums)]
    let _ = discriminant::<i32>(&123); // { dg-error "" "" { target *-*-* } }
}


// ################## Expressions
fn expressions() {
    let closure = |#[deny(non_snake_case)] PARAM: i32| {}; // { dg-error "" "" { target *-*-* } }

    struct Match{f1: i32}
    // I can't find any late lints for patterns.
    // let f = Match{#[deny()]f1: 123};

    let f = Match{f1: 123};
    match f {
        #![deny(enum_intrinsics_non_enums)]
        Match{f1} => {
            discriminant::<i32>(&123); // { dg-error "" "" { target *-*-* } }
        }
    }
    match f {
        #[deny(enum_intrinsics_non_enums)]
        Match{f1} => {
            discriminant::<i32>(&123); // { dg-error "" "" { target *-*-* } }
        }
    }

    match 123 {
        #[deny(non_snake_case)]
        ARM_VAR => {} // { dg-error "" "" { target *-*-* } }
    }

    // Statement Block
    {
        #![deny(enum_intrinsics_non_enums)]
        discriminant::<i32>(&123); // { dg-error "" "" { target *-*-* } }
    }
    let block_tail: () = {
        #[deny(enum_intrinsics_non_enums)]
        discriminant::<i32>(&123); // { dg-error "" "" { target *-*-* } }
    };

    // Before expression as a statement.
    #[deny(enum_intrinsics_non_enums)]
    discriminant::<i32>(&123); // { dg-error "" "" { target *-*-* } }

    [#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)]; // { dg-error "" "" { target *-*-* } }
    (#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123),); // { dg-error "" "" { target *-*-* } }
    fn call(p: Discriminant<i32>) {}
    call(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); // { dg-error "" "" { target *-*-* } }
    struct TupleStruct(Discriminant<i32>);
    TupleStruct(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); // { dg-error "" "" { target *-*-* } }
}


// ################## Patterns
fn patterns() {
    // There aren't any late lints that I can find that apply to pattern fields.
    //
    // struct PatField{f1: i32, f2: i32};
    // let f = PatField{f1: 1, f2: 2};
    // let PatField{#[deny()]f1, #[deny()]..} = f;
}

fn main() {}

