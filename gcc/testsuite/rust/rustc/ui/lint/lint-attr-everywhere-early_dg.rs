// Tests that lint levels can be set for early lints.
#![allow(non_camel_case_types, unsafe_code, while_true, unused_parens)]

// The following is a check of the lints used here to verify they do not warn
// when allowed.
fn verify_no_warnings() {
    type non_camel_type = i32; // non_camel_case_types
    struct NON_CAMEL_IS_ALLOWED; // non_camel_case_types
    unsafe {} // unsafe_code
    enum Enum {
        VARIANT_CAMEL // non_camel_case_types
    }
    fn generics<foo>() {} // non_camel_case_types
    while true {} // while_true
    type T = (i32); // unused_parens
}


// ################## Types

#[deny(non_camel_case_types)]
type type_outer = i32; // { dg-error "" "" { target *-*-* } }

type BareFnPtr = fn(#[deny(unused_parens)](i32)); // { dg-error "" "" { target *-*-* } }
// There aren't any early lints that currently apply to the variadic spot.
// type BareFnPtrVariadic = extern "C" fn(i32, #[deny()]...);

// ################## Items
#[deny(non_camel_case_types)]
struct ITEM_OUTER; // { dg-error "" "" { target *-*-* } }

mod module_inner {
    #![deny(unsafe_code)]
    fn f() {
        unsafe {} // { dg-error "" "" { target *-*-* } }
    }
}

struct Associated;
impl Associated {
    #![deny(unsafe_code)]

    fn inherent_denied_from_inner() { unsafe {} } // { dg-error "" "" { target *-*-* } }

    #[deny(while_true)]
    fn inherent_fn() { while true {} } // { dg-error "" "" { target *-*-* } }

    #[deny(while_true)]
    const INHERENT_CONST: i32 = {while true {} 1}; // { dg-error "" "" { target *-*-* } }
}

trait trait_inner { // { dg-error "" "" { target *-*-* } }
    #![deny(non_camel_case_types)]
}

trait AssociatedTrait {
    #![deny(unsafe_code)]

    fn denied_from_inner() { unsafe {} } // { dg-error "" "" { target *-*-* } }

    #[deny(while_true)]
    fn assoc_fn() { while true {} } // { dg-error "" "" { target *-*-* } }

    #[deny(while_true)]
    const ASSOC_CONST: i32 = {while true {} 1}; // { dg-error "" "" { target *-*-* } }

    #[deny(non_camel_case_types)]
    type assoc_type; // { dg-error "" "" { target *-*-* } }
}

impl AssociatedTrait for Associated {
    #![deny(unsafe_code)]

    fn denied_from_inner() { unsafe {} } // { dg-error "" "" { target *-*-* } }

    #[deny(while_true)]
    fn assoc_fn() { while true {} } // { dg-error "" "" { target *-*-* } }

    #[deny(while_true)]
    const ASSOC_CONST: i32 = {while true {} 1};  // { dg-error "" "" { target *-*-* } }

    #[deny(unused_parens)]
    type assoc_type = (i32); // { dg-error "" "" { target *-*-* } }
}

struct StructFields {
    #[deny(unused_parens)]f1: (i32), // { dg-error "" "" { target *-*-* } }
}
struct StructTuple(#[deny(unused_parens)](i32)); // { dg-error "" "" { target *-*-* } }

enum Enum {
    #[deny(non_camel_case_types)]
    VARIANT_CAMEL, // { dg-error "" "" { target *-*-* } }
}

extern "C" {
    #![deny(unused_parens)]

    fn foreign_denied_from_inner(x: (i32)); // { dg-error "" "" { target *-*-* } }
}

extern "C" {
    #[deny(unused_parens)]
    fn foreign_denied_from_outer(x: (i32)); // { dg-error "" "" { target *-*-* } }
}

fn function(#[deny(unused_parens)] param: (i32)) {} // { dg-error "" "" { target *-*-* } }

fn generics<#[deny(non_camel_case_types)]foo>() {} // { dg-error "" "" { target *-*-* } }


// ################## Statements
fn statements() {
    #[deny(unused_parens)]
    let x = (1); // { dg-error "" "" { target *-*-* } }
}


// ################## Expressions
fn expressions() {
    let closure = |#[deny(unused_parens)] param: (i32)| {}; // { dg-error "" "" { target *-*-* } }

    struct Match{f1: i32}
    // Strangely unused_parens doesn't fire with {f1: (123)}
    let f = Match{#[deny(unused_parens)]f1: {(123)}}; // { dg-error "" "" { target *-*-* } }

    match f {
        #![deny(unsafe_code)]

        #[deny(while_true)]
        Match{f1} => {
            unsafe {} // { dg-error "" "" { target *-*-* } }
            while true {} // { dg-error "" "" { target *-*-* } }
        }
    }

    match f {
        #[deny(ellipsis_inclusive_range_patterns)]
        Match{f1: 0...100} => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        _ => {}
    }

    // Statement Block
    {
        #![deny(unsafe_code)]
        unsafe {} // { dg-error "" "" { target *-*-* } }
    }
    let block_tail: () = {
        #[deny(unsafe_code)]
        unsafe {} // { dg-error "" "" { target *-*-* } }
    };

    // Before expression as a statement.
    #[deny(unsafe_code)]
    unsafe {}; // { dg-error "" "" { target *-*-* } }

    [#[deny(unsafe_code)] unsafe {123}]; // { dg-error "" "" { target *-*-* } }
    (#[deny(unsafe_code)] unsafe {123},); // { dg-error "" "" { target *-*-* } }
    fn call(p: i32) {}
    call(#[deny(unsafe_code)] unsafe {123}); // { dg-error "" "" { target *-*-* } }
    struct TupleStruct(i32);
    TupleStruct(#[deny(unsafe_code)] unsafe {123}); // { dg-error "" "" { target *-*-* } }
}


// ################## Patterns
fn patterns() {
    struct PatField{f1: i32, f2: i32};
    let f = PatField{f1: 1, f2: 2};
    match f {
        PatField {
            #[deny(ellipsis_inclusive_range_patterns)]
            f1: 0...100,
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
            ..
        } => {}
        _ => {}
    }
}

fn main() {}

