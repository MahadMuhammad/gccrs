// Test that we don't prepend `::` to paths referencing crates from the extern prelude
// when it can be avoided[^1] since it's more idiomatic to do so.
//
// [^1]: Counterexample: `unresolved-import-suggest-disambiguated-crate-name.rs`
#![feature(decl_macro)] // allows us to create items with hygienic names

//@ aux-crate:library=library.rs
// { dg-additional-options "-frust-edition= 2021" }

mod hygiene {
    make!();
    macro make() {
        // This won't conflict with the suggested *non-global* path as the syntax context differs.
        mod library {}
    }

    mod module {}
    use module::SomeUsefulType; // { dg-error ".E0432." "" { target *-*-* } }
}

mod glob {
    use inner::*;
    mod inner {
        mod library {}
    }

    mod module {}
    use module::SomeUsefulType; // { dg-error ".E0432." "" { target *-*-* } }
}

fn main() {}

