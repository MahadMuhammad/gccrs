#[cfg(target(os = "linux"))] // { dg-error ".E0658." "" { target *-*-* } }
struct Foo(u64, u64);

#[cfg_attr(target(os = "linux"), non_exhaustive)] // { dg-error ".E0658." "" { target *-*-* } }
struct Bar(u64, u64);

#[cfg(not(any(all(target(os = "linux")))))] // { dg-error ".E0658." "" { target *-*-* } }
fn foo() {}

fn main() {
    cfg!(target(os = "linux"));
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

