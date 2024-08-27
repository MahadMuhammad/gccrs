// { dg-additional-options "-frust-edition=2021" }
#![feature(stmt_expr_attributes)]
#![feature(coroutines)]

fn main() {
    let _closure = #[track_caller] || {}; // { dg-error ".E0658." "" { target *-*-* } }
    let _coroutine = #[coroutine] #[track_caller] || { yield; }; // { dg-error ".E0658." "" { target *-*-* } }
    let _future = #[track_caller] async {}; // { dg-error ".E0658." "" { target *-*-* } }
}

