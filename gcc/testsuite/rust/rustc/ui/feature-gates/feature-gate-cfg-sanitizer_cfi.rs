#[cfg(sanitizer_cfi_generalize_pointers)]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn foo() {}

#[cfg(sanitizer_cfi_normalize_integers)]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() {}

fn main() {}

