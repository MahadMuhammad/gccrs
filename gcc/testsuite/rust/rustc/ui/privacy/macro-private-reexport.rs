// { dg-additional-options "-frust-edition=2021" }

#![feature(decl_macro)]

mod foo {
    macro_rules! bar {
        () => {};
    }

    pub use bar as _; // { dg-error ".E0364." "" { target *-*-* } }

    macro baz() {}

    pub use baz as _; // { dg-error ".E0364." "" { target *-*-* } }
}

fn main() {}

