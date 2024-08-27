#![feature(dyn_star)]
// { dg-warning "" "" { target *-*-* } .-1 }

union Union {
    x: usize,
}

trait Trait {}
impl Trait for Union {}

fn bar(_: dyn* Trait) {}

fn main() {
    bar(Union { x: 0usize });
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

