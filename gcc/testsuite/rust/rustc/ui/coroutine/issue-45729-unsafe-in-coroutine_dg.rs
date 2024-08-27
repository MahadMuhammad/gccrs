#![feature(coroutines, stmt_expr_attributes)]

fn main() {
    let _ = #[coroutine]
    || {
        *(1 as *mut u32) = 42;
// { dg-error ".E0133." "" { target *-*-* } .-1 }
        yield;
    };
}

