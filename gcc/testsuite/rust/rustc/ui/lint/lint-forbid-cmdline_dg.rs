//@ compile-flags: -F deprecated

#[allow(deprecated)] // { dg-error ".E0453." "" { target *-*-* } }
fn main() {
}

