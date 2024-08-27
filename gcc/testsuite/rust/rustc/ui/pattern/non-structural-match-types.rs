// { dg-additional-options "-frust-edition=2021" }

#![allow(unreachable_code)]
#![feature(const_async_blocks)]
#![feature(inline_const_pat)]

fn main() {
    match loop {} {
        const { || {} } => {} // { dg-error "" "" { target *-*-* } }
    }
    match loop {} {
        const { async {} } => {} // { dg-error "" "" { target *-*-* } }
    }
}

