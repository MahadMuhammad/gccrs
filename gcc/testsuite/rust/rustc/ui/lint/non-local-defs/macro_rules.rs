//@ check-pass
// { dg-additional-options "-frust-edition=2021" }
//@ aux-build:non_local_macro.rs
//@ rustc-env:CARGO_CRATE_NAME=non_local_def

#![warn(non_local_definitions)]

extern crate non_local_macro;

const B: u32 = {
    #[macro_export]
    macro_rules! m0 { () => { } };
// { dg-warning "" "" { target *-*-* } .-1 }

    1
};

non_local_macro::non_local_macro_rules!(my_macro);
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {
    #[macro_export]
    macro_rules! m { () => { } };
// { dg-warning "" "" { target *-*-* } .-1 }

    struct InsideMain;

    impl InsideMain {
        fn bar() {
            #[macro_export]
            macro_rules! m2 { () => { } };
// { dg-warning "" "" { target *-*-* } .-1 }
        }
    }
}

