//@ revisions: with-generic-asset without-generic-asset
//@ [with-generic-asset] compile-flags: --cfg feature="generic_assert"

fn main() {
    assert!();  // { dg-error "" "" { target *-*-* } }
    assert!(struct); // { dg-error "" "" { target *-*-* } }
    debug_assert!(); // { dg-error "" "" { target *-*-* } }
    debug_assert!(struct); // { dg-error "" "" { target *-*-* } }
}

