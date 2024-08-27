#[deprecated_safe(since = "TBD", note = "...")] // { dg-error ".E0658." "" { target *-*-* } }
unsafe fn deprecated_safe_fn() {}

#[deprecated_safe(since = "TBD", note = "...")] // { dg-error ".E0658." "" { target *-*-* } }
unsafe trait DeprecatedSafeTrait {}

fn main() {}

