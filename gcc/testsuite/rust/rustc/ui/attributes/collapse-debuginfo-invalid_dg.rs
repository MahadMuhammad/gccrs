#![feature(stmt_expr_attributes)]
#![feature(type_alias_impl_trait)]
#![no_std]

// Test that the `#[collapse_debuginfo]` attribute can only be used on macro definitions.

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
extern crate std;

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
use std::collections::HashMap;

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
static FOO: u32 = 3;

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
const BAR: u32 = 3;

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
fn foo() {
    let _ = #[collapse_debuginfo(yes)] || { };
// { dg-error "" "" { target *-*-* } .-1 }
    #[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = 3;
    let _ = #[collapse_debuginfo(yes)] 3;
// { dg-error "" "" { target *-*-* } .-1 }
    match (3, 4) {
        #[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
        _ => (),
    }
}

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
mod bar {
}

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
type Map = HashMap<u32, u32>;

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
enum Foo {
    #[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
    Variant,
}

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
struct Bar {
    #[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
    field: u32,
}

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
union Qux {
    a: u32,
    b: u16
}

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
trait Foobar {
    #[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
    type Bar;
}

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
type AFoobar = impl Foobar;

impl Foobar for Bar {
    type Bar = u32;
}

fn constraining() -> AFoobar {
    Bar { field: 3 }
}

#[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
impl Bar {
    #[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
    const FOO: u32 = 3;

    #[collapse_debuginfo(yes)]
// { dg-error "" "" { target *-*-* } .-1 }
    fn bar(&self) {}
}

#[collapse_debuginfo(yes)]
macro_rules! finally {
    ($e:expr) => { $e }
}

fn main() {}

