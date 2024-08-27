#![feature(marker_trait_attr)]

#[marker]
trait Marker {
    const N: usize = 0;
// { dg-error ".E0714." "" { target *-*-* } .-1 }
    fn do_something() {}
// { dg-error ".E0714." "" { target *-*-* } .-1 }
}

struct OverrideConst;
impl Marker for OverrideConst {
// { dg-error ".E0715." "" { target *-*-* } .-1 }
    const N: usize = 1;
}

struct OverrideFn;
impl Marker for OverrideFn {
// { dg-error ".E0715." "" { target *-*-* } .-1 }
    fn do_something() {
        println!("Hello world!");
    }
}

fn main() {}

