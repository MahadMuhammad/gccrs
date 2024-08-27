#![feature(staged_api, never_type, rust_cold_cc)]
// { dg-error "" "" { target *-*-* } .-1 }

#[stable(feature = "a", since = "3.3.3")]
struct StableType;

#[unstable(feature = "b", issue = "none")]
struct UnstableType;

#[stable(feature = "c", since = "3.3.3")]
trait StableTrait {}

#[unstable(feature = "d", issue = "none")]
trait UnstableTrait {}

#[unstable(feature = "e", issue = "none")]
impl UnstableTrait for UnstableType {}

#[unstable(feature = "f", issue = "none")]
impl StableTrait for UnstableType {}

#[unstable(feature = "g", issue = "none")]
impl UnstableTrait for StableType {}

#[unstable(feature = "h", issue = "none")]
impl StableTrait for ! {}

// Note: If rust_cold_cc is stabilized, switch this to another (unstable) ABI.
#[unstable(feature = "i", issue = "none")]
impl StableTrait for extern "rust-cold" fn() {}

#[unstable(feature = "j", issue = "none")]
// { dg-error "" "" { target *-*-* } .-1 }
impl StableTrait for StableType {}

#[unstable(feature = "k", issue = "none")]
// { dg-error "" "" { target *-*-* } .-1 }
impl StableTrait for fn() -> ! {}

#[unstable(feature = "l", issue = "none")]
impl StableTrait for fn() -> UnstableType {}

fn main() {}

