//@ normalize-stderr-test: "`: .*" -> "`: $$FILE_NOT_FOUND_MSG"

// test that errors in a (selection) of macros don't kill compilation
// immediately, so that we get more errors listed at a time.

#![feature(trace_macros, concat_idents)]
#![feature(stmt_expr_attributes)]

use std::arch::asm;

#[derive(Default)]
struct DefaultInnerAttrStruct {
    #[default] // { dg-error "" "" { target *-*-* } }
    foo: (),
}

#[derive(Default)]
struct DefaultInnerAttrTupleStruct(#[default] ());
// { dg-error "" "" { target *-*-* } .-1 }

#[derive(Default)]
#[default] // { dg-error "" "" { target *-*-* } }
struct DefaultOuterAttrStruct {}

#[derive(Default)]
#[default] // { dg-error "" "" { target *-*-* } }
enum DefaultOuterAttrEnum {
    #[default]
    Foo,
}

#[rustfmt::skip] // needs some work to handle this case
#[repr(u8)]
#[derive(Default)]
enum AttrOnInnerExpression {
    Foo = #[default] 0, // { dg-error "" "" { target *-*-* } }
    Bar([u8; #[default] 1]), // { dg-error "" "" { target *-*-* } }
    #[default]
    Baz,
}

#[derive(Default)] // { dg-error "" "" { target *-*-* } }
enum NoDeclaredDefault {
    Foo,
    Bar,
}

#[derive(Default)] // { dg-error "" "" { target *-*-* } }
enum MultipleDefaults {
    #[default]
    Foo,
    #[default]
    Bar,
    #[default]
    Baz,
}

#[derive(Default)]
enum ExtraDeriveTokens {
    #[default = 1] // { dg-error "" "" { target *-*-* } }
    Foo,
}

#[derive(Default)]
enum TwoDefaultAttrs {
    #[default]
    #[default]
    Foo, // { dg-error "" "" { target *-*-* } }
    Bar,
}

#[derive(Default)]
enum ManyDefaultAttrs {
    #[default]
    #[default]
    #[default]
    #[default]
    Foo, // { dg-error "" "" { target *-*-* } }
    Bar,
}

#[derive(Default)]
enum DefaultHasFields {
    #[default]
    Foo {}, // { dg-error "" "" { target *-*-* } }
    Bar,
}

#[derive(Default)]
enum NonExhaustiveDefault {
    #[default]
    #[non_exhaustive]
    Foo, // { dg-error "" "" { target *-*-* } }
    Bar,
}

fn main() {
    asm!(invalid); // { dg-error "" "" { target *-*-* } }
    llvm_asm!(invalid); // { dg-error "" "" { target *-*-* } }

    concat_idents!("not", "idents"); // { dg-error "" "" { target *-*-* } }

    option_env!(invalid); // { dg-error "" "" { target *-*-* } }
    env!(invalid); // { dg-error "" "" { target *-*-* } }
    env!(foo, abr, baz); // { dg-error "" "" { target *-*-* } }
    env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST"); // { dg-error "" "" { target *-*-* } }

    format!(invalid); // { dg-error "" "" { target *-*-* } }

    include!(invalid); // { dg-error "" "" { target *-*-* } }

    include_str!(invalid); // { dg-error "" "" { target *-*-* } }
    include_str!("i'd be quite surprised if a file with this name existed"); // { dg-error "" "" { target *-*-* } }
    include_bytes!(invalid); // { dg-error "" "" { target *-*-* } }
    include_bytes!("i'd be quite surprised if a file with this name existed"); // { dg-error "" "" { target *-*-* } }

    trace_macros!(invalid); // { dg-error "" "" { target *-*-* } }
}

/// Check that `#[derive(Default)]` does use a `T : Default` bound when the
/// `#[default]` variant is `#[non_exhaustive]` (should this end up allowed).
const _: () = {
    #[derive(Default)]
    enum NonExhaustiveDefaultGeneric<T> {
        #[default]
        #[non_exhaustive]
        Foo, // { dg-error "" "" { target *-*-* } }
        Bar(T),
    }

    fn assert_impls_default<T: Default>() {}

    enum NotDefault {}

    // Note: the `derive(Default)` currently bails early enough for trait-checking
    // not to happen. Should it bail late enough, or even pass, make sure to
    // assert that the following line fails.
    let _ = assert_impls_default::<NonExhaustiveDefaultGeneric<NotDefault>>;
};

