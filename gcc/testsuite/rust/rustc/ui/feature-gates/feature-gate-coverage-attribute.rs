#![crate_type = "lib"]
#![feature(no_coverage)] // { dg-error ".E0557." "" { target *-*-* } }

#[derive(PartialEq, Eq)] // ensure deriving `Eq` does not enable `feature(coverage)`
struct Foo {
    a: u8,
    b: u32,
}

#[coverage(off)] // { dg-error ".E0658." "" { target *-*-* } }
fn requires_feature_coverage() -> bool {
    let bar = Foo { a: 0, b: 0 };
    bar == Foo { a: 0, b: 0 }
}

