#![feature(stmt_expr_attributes)]
#![feature(custom_test_frameworks)]

fn main() {
    let _ = #[cfg(FALSE)] ();
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = 1 + 2 + #[cfg(FALSE)] 3;
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = [1, 2, 3][#[cfg(FALSE)] 1];
// { dg-error "" "" { target *-*-* } .-1 }
}

