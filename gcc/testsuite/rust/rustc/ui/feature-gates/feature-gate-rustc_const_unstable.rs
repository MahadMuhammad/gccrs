// Test internal const fn feature gate.

#[rustc_const_unstable(feature="fzzzzzt")] // { dg-error ".E0734." "" { target *-*-* } }
pub const fn bazinga() {}

fn main() {
}

