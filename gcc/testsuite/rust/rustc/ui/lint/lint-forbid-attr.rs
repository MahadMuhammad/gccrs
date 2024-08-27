#![forbid(deprecated)]

#[allow(deprecated)]
// { dg-error ".E0453." "" { target *-*-* } .-1 }
fn main() {
}

