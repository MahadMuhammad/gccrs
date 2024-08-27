// gate-test-coroutine_clone
// Verifies that static coroutines cannot be cloned/copied.

#![feature(coroutines, coroutine_clone, stmt_expr_attributes)]

fn main() {
    let gen = #[coroutine]
    static move || {
        yield;
    };
    check_copy(&gen);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_clone(&gen);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn check_copy<T: Copy>(_x: &T) {}
fn check_clone<T: Clone>(_x: &T) {}

