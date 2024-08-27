#![allow(dead_code, path_statements)]
#![deny(unused_attributes, unused_must_use)]
#![feature(asm_experimental_arch, stmt_expr_attributes, trait_alias)]

#[must_use] // { dg-error "" "" { target *-*-* } }
extern crate std as std2;

#[must_use] // { dg-error "" "" { target *-*-* } }
mod test_mod {}

#[must_use] // { dg-error "" "" { target *-*-* } }
use std::arch::global_asm;

#[must_use] // { dg-error "" "" { target *-*-* } }
const CONST: usize = 4;
#[must_use] // { dg-error "" "" { target *-*-* } }
#[no_mangle]
static STATIC: usize = 4;

#[must_use]
struct X;

#[must_use]
enum Y {
    Z,
}

#[must_use]
union U {
    unit: (),
}

#[must_use] // { dg-error "" "" { target *-*-* } }
impl U {
    #[must_use]
    fn method() -> i32 {
        4
    }
}

#[must_use]
#[no_mangle]
fn foo() -> i64 {
    4
}

#[must_use] // { dg-error "" "" { target *-*-* } }
extern "Rust" {
    #[link_name = "STATIC"]
    #[must_use] // { dg-error "" "" { target *-*-* } }
    static FOREIGN_STATIC: usize;

    #[link_name = "foo"]
    #[must_use]
    fn foreign_foo() -> i64;
}

#[must_use] // { dg-error "" "" { target *-*-* } }
global_asm!("");

#[must_use] // { dg-error "" "" { target *-*-* } }
type UseMe = ();

fn qux<#[must_use] T>(_: T) {} // { dg-error "" "" { target *-*-* } }

#[must_use]
trait Use {
    #[must_use] // { dg-error "" "" { target *-*-* } }
    const ASSOC_CONST: usize = 4;
    #[must_use] // { dg-error "" "" { target *-*-* } }
    type AssocTy;

    #[must_use]
    fn get_four(&self) -> usize {
        4
    }
}

#[must_use] // { dg-error "" "" { target *-*-* } }
impl Use for () {
    type AssocTy = ();
}

#[must_use] // { dg-error "" "" { target *-*-* } }
trait Alias = Use;

#[must_use] // { dg-error "" "" { target *-*-* } }
macro_rules! cool_macro {
    () => {
        4
    };
}

fn main() {
    #[must_use] // { dg-error "" "" { target *-*-* } }
    let x = || {};
    x();

    let x = #[must_use] // { dg-error "" "" { target *-*-* } }
    || {};
    x();

    X; // { dg-error "" "" { target *-*-* } }
    Y::Z; // { dg-error "" "" { target *-*-* } }
    U { unit: () }; // { dg-error "" "" { target *-*-* } }
    U::method(); // { dg-error "" "" { target *-*-* } }
    foo(); // { dg-error "" "" { target *-*-* } }

    unsafe {
        foreign_foo(); // { dg-error "" "" { target *-*-* } }
    };

    CONST;
    STATIC;
    unsafe { FOREIGN_STATIC };
    cool_macro!();
    qux(4);
    ().get_four(); // { dg-error "" "" { target *-*-* } }

    match Some(4) {
        #[must_use] // { dg-error "" "" { target *-*-* } }
        Some(res) => res,
        None => 0,
    };

    struct PatternField {
        foo: i32,
    }
    let s = PatternField { #[must_use]  foo: 123 }; // { dg-error "" "" { target *-*-* } }
    let PatternField { #[must_use] foo } = s; // { dg-error "" "" { target *-*-* } }
}

