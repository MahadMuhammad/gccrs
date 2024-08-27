#![feature(optimize_attribute)]
#![feature(stmt_expr_attributes)]
#![deny(unused_attributes)]
#![allow(dead_code)]

#[optimize(speed)] // { dg-error "" "" { target *-*-* } }
struct F;

fn invalid() {
    #[optimize(speed)] // { dg-error "" "" { target *-*-* } }
    {
        1
    };
}

#[optimize(speed)]
fn valid() {}

#[optimize(speed)]
mod valid_module {}

#[optimize(speed)]
impl F {}

fn main() {
    let _ = #[optimize(speed)]
    (|| 1);
}

