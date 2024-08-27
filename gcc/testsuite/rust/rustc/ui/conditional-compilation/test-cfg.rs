//@ compile-flags: --cfg foo --check-cfg=cfg(foo,bar)

#[cfg(all(foo, bar))] // foo AND bar
fn foo() {}

fn main() {
    foo(); // { dg-error ".E0425." "" { target *-*-* } }
}

