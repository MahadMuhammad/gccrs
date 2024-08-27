// Tests that subsequent lints specified via the command line override
// each other, except for ForceWarn and Forbid, which cannot be overridden.
//
//@ revisions: warn_deny forbid_warn force_warn_deny
//
//@[warn_deny] compile-flags: --warn missing_abi --deny missing_abi
//@[forbid_warn] compile-flags: --warn missing_abi --forbid missing_abi
//@[force_warn_deny] compile-flags: --force-warn missing_abi --allow missing_abi
//@[force_warn_deny] check-pass


extern fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }

fn main() {}

