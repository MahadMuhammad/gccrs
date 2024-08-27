//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@ compile-flags: -Zverbose-internals
//@[next] compile-flags: -Znext-solver
//@ normalize-stderr-test: "DefId\([^\)]+\)" -> "DefId(..)"

#![feature(rustc_attrs)]
#![rustc_hidden_type_of_opaques]

// Make sure that the compiler can handle `ReErased` in the hidden type of an opaque.

fn foo<'a: 'a>(x: &'a Vec<i32>) -> impl Fn() + 'static {
// { dg-error "" "" { target *-*-* } .-1 }
    // Can't write whole type because of lack of path sanitization
    || ()
}

fn bar() -> impl Fn() + 'static {
// { dg-error "" "" { target *-*-* } .-1 }
    // Can't write whole type because of lack of path sanitization
    foo(&vec![])
}

fn main() {}

