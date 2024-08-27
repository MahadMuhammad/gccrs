//@ only-x86_64

#![warn(unused_attributes)]

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
extern crate alloc;
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
use alloc::alloc::alloc;
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
extern "Rust" {}
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature = "+sse2"]
// { dg-error "" "" { target *-*-* } .-1 }
#[target_feature(enable = "foo")]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
#[target_feature(bar)]
// { dg-error "" "" { target *-*-* } .-1 }
#[target_feature(disable = "baz")]
// { dg-error "" "" { target *-*-* } .-1 }
unsafe fn foo() {}

#[target_feature(enable = "sse2")]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
fn bar() {}
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
mod another {}
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
const FOO: usize = 7;
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
struct Foo;
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
enum Bar {}
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
union Qux {
// { dg-note "" "" { target *-*-* } .-1 }
    f1: u16,
    f2: u16,
}

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
type Uwu = ();
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
trait Baz {}
// { dg-note "" "" { target *-*-* } .-1 }

#[inline(always)]
// { dg-error "" "" { target *-*-* } .-1 }
#[target_feature(enable = "sse2")]
unsafe fn test() {}

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
static A: () = ();
// { dg-note "" "" { target *-*-* } .-1 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
impl Quux for u8 {}
// { dg-note ".E0046." "" { target *-*-* } .-1 }
// { dg-note ".E0046." "" { target *-*-* } .-2 }
// { dg-error ".E0046." "" { target *-*-* } .-3 }

#[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
impl Foo {}
// { dg-note "" "" { target *-*-* } .-1 }

trait Quux {
    fn foo(); // { dg-note "" "" { target *-*-* } }
}

impl Quux for Foo {
    #[target_feature(enable = "sse2")]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    fn foo() {}
// { dg-note "" "" { target *-*-* } .-1 }
}

fn main() {
    #[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
    unsafe {
        foo();
        bar();
    }
// { dg-note "" "" { target *-*-* } .-4 }

    #[target_feature(enable = "sse2")]
// { dg-error "" "" { target *-*-* } .-1 }
    || {};
// { dg-note "" "" { target *-*-* } .-1 }
}

