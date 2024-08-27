#![feature(cfg_eval)]
#![feature(stmt_expr_attributes)]

fn main() {
    let _ = #[cfg_eval] #[cfg(FALSE)] 0;
// { dg-error "" "" { target *-*-* } .-1 }
}

