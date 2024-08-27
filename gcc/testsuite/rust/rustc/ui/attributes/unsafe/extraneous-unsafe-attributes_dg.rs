// { dg-additional-options "-frust-edition= 2024" }
//@ compile-flags: -Zunstable-options

#[unsafe(cfg(any()))] // { dg-error "" "" { target *-*-* } }
fn a() {}

#[unsafe(cfg_attr(any(), allow(dead_code)))] // { dg-error "" "" { target *-*-* } }
fn b() {}

#[unsafe(test)] // { dg-error "" "" { target *-*-* } }
fn aa() {}

#[unsafe(ignore = "test")] // { dg-error "" "" { target *-*-* } }
fn bb() {}

#[unsafe(should_panic(expected = "test"))] // { dg-error "" "" { target *-*-* } }
fn cc() {}

#[unsafe(macro_use)] // { dg-error "" "" { target *-*-* } }
mod inner {
    #[unsafe(macro_export)] // { dg-error "" "" { target *-*-* } }
    macro_rules! m {
        () => {};
    }
}

#[unsafe(used)] // { dg-error "" "" { target *-*-* } }
static FOO: usize = 0;

fn main() {}

