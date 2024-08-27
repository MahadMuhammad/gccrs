#![feature(coverage_attribute)]
// { dg-additional-options "-frust-edition= 2021" }

// Demonstrates the diagnostics produced when using the syntax
// `#[coverage = "off"]`, which should not be allowed.
//
// The syntax is tested both in places that can have a coverage attribute,
// and in places that cannot have a coverage attribute, to demonstrate the
// interaction between multiple errors.

#[coverage = "off"]
// { dg-error "" "" { target *-*-* } .-1 }
mod my_mod {}

mod my_mod_inner {
    #![coverage = "off"]
// { dg-error "" "" { target *-*-* } .-1 }
}

#[coverage = "off"]
// { dg-error ".E0788." "" { target *-*-* } .-1 }
// { dg-error ".E0788." "" { target *-*-* } .-2 }
struct MyStruct;

#[coverage = "off"]
// { dg-error "" "" { target *-*-* } .-1 }
impl MyStruct {
    #[coverage = "off"]
// { dg-error ".E0788." "" { target *-*-* } .-1 }
// { dg-error ".E0788." "" { target *-*-* } .-2 }
    const X: u32 = 7;
}

#[coverage = "off"]
// { dg-error ".E0788." "" { target *-*-* } .-1 }
// { dg-error ".E0788." "" { target *-*-* } .-2 }
trait MyTrait {
    #[coverage = "off"]
// { dg-error ".E0788." "" { target *-*-* } .-1 }
// { dg-error ".E0788." "" { target *-*-* } .-2 }
    const X: u32;

    #[coverage = "off"]
// { dg-error ".E0788." "" { target *-*-* } .-1 }
// { dg-error ".E0788." "" { target *-*-* } .-2 }
    type T;
}

#[coverage = "off"]
// { dg-error "" "" { target *-*-* } .-1 }
impl MyTrait for MyStruct {
    #[coverage = "off"]
// { dg-error ".E0788." "" { target *-*-* } .-1 }
// { dg-error ".E0788." "" { target *-*-* } .-2 }
    const X: u32 = 8;

    #[coverage = "off"]
// { dg-error ".E0788." "" { target *-*-* } .-1 }
// { dg-error ".E0788." "" { target *-*-* } .-2 }
    type T = ();
}

#[coverage = "off"]
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {}

