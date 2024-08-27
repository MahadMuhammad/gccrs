//@ revisions: e2024 none
//@[e2024] compile-flags: --edition 2024 -Zunstable-options
#![cfg_attr(e2024, feature(gen_blocks))]
#![feature(stmt_expr_attributes)]

fn main() {
    let x = gen {};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    let y = gen { yield 42 };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    gen {};
// { dg-error "" "" { target *-*-* } .-1 }

    let _ = || yield true; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    let _ = #[coroutine] || yield true; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    let _ = #[coroutine] || {};
// { dg-error "" "" { target *-*-* } .-1 }
}

