#[cfg(version(42))] // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn foo() {}
#[cfg(version(1.20))] // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn foo() -> bool { true }
#[cfg(version("1.44"))]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn foo() -> bool { true }
#[cfg(not(version("1.44")))]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn foo() -> bool { false }

#[cfg(version("1.43", "1.44", "1.45"))] // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool  { false }
#[cfg(version(false))] // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool  { false }
#[cfg(version("foo"))] // { dg-warning ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool  { false }
#[cfg(version("999"))] // { dg-warning ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool  { false }
#[cfg(version("-1"))] // { dg-warning ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool  { false }
#[cfg(version("65536"))] // { dg-warning ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool  { false }
#[cfg(version("0"))] // { dg-warning ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool { true }
#[cfg(version("1.0"))]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool { true }
#[cfg(version("1.65536.2"))] // { dg-warning ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() -> bool  { false }
#[cfg(version("1.20.0-stable"))] // { dg-warning ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn bar() {}

fn main() {
    assert!(foo());
    assert!(bar());
    assert!(cfg!(version("1.42"))); // { dg-error ".E0658." "" { target *-*-* } }
}

