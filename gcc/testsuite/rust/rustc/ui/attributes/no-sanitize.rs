#![feature(no_sanitize)]
#![feature(stmt_expr_attributes)]
#![deny(unused_attributes)]
#![allow(dead_code)]

fn invalid() {
    #[no_sanitize(memory)] // { dg-error "" "" { target *-*-* } }
    {
        1
    };
}

#[no_sanitize(memory)] // { dg-error "" "" { target *-*-* } }
type InvalidTy = ();

#[no_sanitize(memory)] // { dg-error "" "" { target *-*-* } }
mod invalid_module {}

fn main() {
    let _ = #[no_sanitize(memory)] // { dg-error "" "" { target *-*-* } }
    (|| 1);
}

#[no_sanitize(memory)] // { dg-error "" "" { target *-*-* } }
struct F;

#[no_sanitize(memory)] // { dg-error "" "" { target *-*-* } }
impl F {
    #[no_sanitize(memory)]
    fn valid(&self) {}
}

#[no_sanitize(memory)]
fn valid() {}

