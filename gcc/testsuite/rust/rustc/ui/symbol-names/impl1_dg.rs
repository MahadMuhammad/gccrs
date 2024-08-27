//@ build-fail
//@ revisions: legacy v0
//@[legacy]compile-flags: -Z unstable-options -C symbol-mangling-version=legacy
    //@[v0]compile-flags: -C symbol-mangling-version=v0
//@[legacy]normalize-stderr-test: "h[\w]{16}E?\)" -> "<SYMBOL_HASH>)"

#![feature(auto_traits, rustc_attrs)]
#![allow(dead_code)]

mod foo {
    pub struct Foo { x: u32 }

    impl Foo {
        #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        #[rustc_def_path]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        fn bar() { }
    }
}

mod bar {
    use foo::Foo;

    impl Foo {
        #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        #[rustc_def_path]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        fn baz() { }
    }
}

trait Foo {
    type Assoc;
}

auto trait AutoTrait {}

fn main() {
    // Test closure mangling, and disambiguators.
    || {};
    || {
        trait Bar {
            fn method(&self) {}
        }

        // Test type mangling, by putting them in an `impl` header.
        impl Bar for [&'_ (dyn Foo<Assoc = extern "C" fn(&u8, ...)> + AutoTrait); 3] {
            #[rustc_symbol_name]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
            #[rustc_def_path]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
            fn method(&self) {}
        }
    };
}

