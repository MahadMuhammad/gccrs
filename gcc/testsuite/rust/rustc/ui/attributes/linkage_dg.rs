#![feature(linkage)]
#![feature(stmt_expr_attributes)]
#![deny(unused_attributes)]
#![allow(dead_code)]

#[linkage = "weak"] // { dg-error "" "" { target *-*-* } }
type InvalidTy = ();

#[linkage = "weak"] // { dg-error "" "" { target *-*-* } }
mod invalid_module {}

#[linkage = "weak"] // { dg-error "" "" { target *-*-* } }
struct F;

#[linkage = "weak"] // { dg-error "" "" { target *-*-* } }
impl F {
    #[linkage = "weak"]
    fn valid(&self) {}
}

#[linkage = "weak"]
fn f() {
    #[linkage = "weak"]
    {
        1
    };
// { dg-error "" "" { target *-*-* } .-4 }
}

extern "C" {
    #[linkage = "weak"]
    static A: *const ();

    #[linkage = "weak"]
    fn bar();
}

fn main() {
    let _ = #[linkage = "weak"]
    (|| 1);
// { dg-error "" "" { target *-*-* } .-2 }
}

