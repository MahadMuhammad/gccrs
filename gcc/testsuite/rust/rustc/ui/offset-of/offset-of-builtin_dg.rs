#![feature(builtin_syntax)]

// For the exposed macro we already test these errors in the other files,
// but this test helps to make sure the builtin construct also errors.
// This has the same examples as offset-of-arg-count.rs

fn main() {
    builtin # offset_of(NotEnoughArguments); // { dg-error "" "" { target *-*-* } }
}
fn t1() {
    builtin # offset_of(NotEnoughArgumentsWithAComma, ); // { dg-error "" "" { target *-*-* } }
}
fn t2() {
    builtin # offset_of(S, f, too many arguments); // { dg-error "" "" { target *-*-* } }
}
fn t3() {
    builtin # offset_of(S, f); // compiles fine
}
fn t4() {
    builtin # offset_of(S, f.); // { dg-error "" "" { target *-*-* } }
}
fn t5() {
    builtin # offset_of(S, f.,); // { dg-error "" "" { target *-*-* } }
}
fn t6() {
    builtin # offset_of(S, f..); // { dg-error "" "" { target *-*-* } }
}
fn t7() {
    builtin # offset_of(S, f..,); // { dg-error "" "" { target *-*-* } }
}

struct S { f: u8, }

