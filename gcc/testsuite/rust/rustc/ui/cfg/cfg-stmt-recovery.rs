// Verify that we do not ICE when failing to parse a statement in `cfg_eval`.

#![feature(cfg_eval)]
#![feature(stmt_expr_attributes)]

#[cfg_eval]
fn main() {
    #[cfg_eval]
    let _ = #[cfg(FALSE)] 0;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

