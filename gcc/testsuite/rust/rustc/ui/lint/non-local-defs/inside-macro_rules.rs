//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

#![warn(non_local_definitions)]

macro_rules! m {
    () => {
        trait MacroTrait {}
        struct OutsideStruct;
        fn my_func() {
            impl MacroTrait for OutsideStruct {}
// { dg-warning "" "" { target *-*-* } .-1 }
        }
    }
}

m!();

fn main() {}

