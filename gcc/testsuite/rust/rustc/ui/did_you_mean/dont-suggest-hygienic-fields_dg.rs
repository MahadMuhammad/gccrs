// Regression test for issue #116334.
// Don't include hygienic fields from different syntax contexts in
// the list of available or similarly named fields.

#![feature(decl_macro)]

macro compound($Ty:ident) {
    #[derive(Default)]
    struct $Ty {
        field: u32, // field `field` is hygienic
    }
}

macro component($Ty:ident) {
    struct $Ty(u64); // field `0` is hygienic (but still accessible via the constructor)
}

compound! { Compound }
component! { Component }

fn main() {
    let ty = Compound::default();

    let _ = ty.field; // { dg-error ".E0609." "" { target *-*-* } }
    let _ = ty.fieeld; // { dg-error ".E0609." "" { target *-*-* } }

    let Compound { field } = ty;
// { dg-error ".E0026." "" { target *-*-* } .-1 }
// { dg-error ".E0026." "" { target *-*-* } .-2 }
// { help ".E0026." "" { target *-*-* } .-3 }

    let ty = Component(90);

    let _ = ty.0; // { dg-error ".E0609." "" { target *-*-* } }
}

environment!();

macro environment() {
    struct Crate { field: () }

    // Here, we do want to suggest `field` even though it's hygienic
    // precisely because they come from the same syntax context.
    const CRATE: Crate = Crate { fiel: () };
// { dg-error ".E0560." "" { target *-*-* } .-1 }
// { help ".E0560." "" { target *-*-* } .-2 }
}

