struct UnitStruct;
struct TupleStruct();
struct BracedStruct{}

enum E {
    UnitVariant,
    TupleVariant(),
    BracedVariant{},
}
use E::*;

const CONST: () = ();
static STATIC: () = ();

fn function() {}

fn main() {
    let doesnt_matter = 0;

    match UnitStruct {
        UnitStruct => {} // OK, `UnitStruct` is a unit struct pattern
    }
    match doesnt_matter {
        TupleStruct => {} // { dg-error ".E0530." "" { target *-*-* } }
    }
    match doesnt_matter {
        BracedStruct => {} // OK, `BracedStruct` is a fresh binding
    }
    match UnitVariant { // { dg-error ".E0004." "" { target *-*-* } }
        UnitVariant => {} // OK, `UnitVariant` is a unit variant pattern
    }
    match doesnt_matter {
        TupleVariant => {} // { dg-error ".E0530." "" { target *-*-* } }
    }
    match doesnt_matter {
        BracedVariant => {} // OK, `BracedVariant` is a fresh binding
    }
    match CONST {
        CONST => {} // OK, `CONST` is a const pattern
    }
    match doesnt_matter {
        STATIC => {} // { dg-error ".E0530." "" { target *-*-* } }
    }
    match doesnt_matter {
        function => {} // OK, `function` is a fresh binding
    }

    let UnitStruct = UnitStruct; // OK, `UnitStruct` is a unit struct pattern
    let TupleStruct = doesnt_matter; // { dg-error ".E0530." "" { target *-*-* } }
    let BracedStruct = doesnt_matter; // OK, `BracedStruct` is a fresh binding
    let UnitVariant = UnitVariant; // { dg-error ".E0005." "" { target *-*-* } }
    let TupleVariant = doesnt_matter; // { dg-error ".E0530." "" { target *-*-* } }
    let BracedVariant = doesnt_matter; // OK, `BracedVariant` is a fresh binding
    let CONST = CONST; // OK, `CONST` is a const pattern
    let STATIC = doesnt_matter; // { dg-error ".E0530." "" { target *-*-* } }
    let function = doesnt_matter; // OK, `function` is a fresh binding
}

