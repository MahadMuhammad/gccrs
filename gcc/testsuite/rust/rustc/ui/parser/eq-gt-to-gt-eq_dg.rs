//@ run-rustfix
// Check that we try to correct `=>` to `>=` in conditions.
#![allow(unused)]

fn main() {
    let a = 0;
    let b = 1;
    if a => b {} // { dg-error "" "" { target *-*-* } }
}

fn foo() {
    let a = 0;
    if a => 1 {} // { dg-error "" "" { target *-*-* } }
}

fn a() {
    let a = 0;
    if 1 => a {} // { dg-error "" "" { target *-*-* } }
}

fn bar() {
    let a = 0;
    let b = 1;
    if a => b && a != b {} // { dg-error "" "" { target *-*-* } }
}

fn qux() {
    let a = 0;
    let b = 1;
    if a != b && a => b {} // { dg-error "" "" { target *-*-* } }
}

fn baz() {
    let a = 0;
    let b = 1;
    let _ = a => b; // { dg-error "" "" { target *-*-* } }
}

fn b() {
    let a = 0;
    let b = 1;
    match a => b { // { dg-error "" "" { target *-*-* } }
        _ => todo!(),
    }
}

