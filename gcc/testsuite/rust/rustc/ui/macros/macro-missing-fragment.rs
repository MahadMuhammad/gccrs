//@ revisions: e2015 e2024
// { dg-additional-options "-frust-edition=2015" }
// { dg-additional-options "-frust-edition=2024" }
//@[e2024] compile-flags: -Zunstable-options

#![warn(missing_fragment_specifier)]

macro_rules! used_arm {
    ( $( any_token $field_rust_type )* ) => {};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
}

macro_rules! used_macro_unused_arm {
    () => {};
    ( $name ) => {};
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

macro_rules! unused_macro {
    ( $name ) => {};
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

fn main() {
    used_arm!();
    used_macro_unused_arm!();
}

