//@ normalize-stderr-test: "(abi|pref|unadjusted_abi_align): Align\([1-8] bytes\)" -> "$1: $$SOME_ALIGN"
//@ normalize-stderr-test: "(size): Size\([48] bytes\)" -> "$1: $$SOME_SIZE"
//@ normalize-stderr-test: "(can_unwind): (true|false)" -> "$1: $$SOME_BOOL"
//@ normalize-stderr-test: "(valid_range): 0\.\.=(4294967295|18446744073709551615)" -> "$1: $$FULL"
// This pattern is prepared for when we account for alignment in the niche.
//@ normalize-stderr-test: "(valid_range): [1-9]\.\.=(429496729[0-9]|1844674407370955161[0-9])" -> "$1: $$NON_NULL"
// Some attributes are only computed for release builds:
//@ compile-flags: -O
#![feature(rustc_attrs)]
#![crate_type = "lib"]

struct S(u16);

#[rustc_abi(debug)]
fn test(_x: u8) -> bool { true } // { dg-error "" "" { target *-*-* } }

#[rustc_abi(debug)]
type TestFnPtr = fn(bool) -> u8; // { dg-error "" "" { target *-*-* } }

#[rustc_abi(debug)]
fn test_generic<T>(_x: *const T) { } // { dg-error "" "" { target *-*-* } }

#[rustc_abi(debug)]
const C: () = (); // { dg-error "" "" { target *-*-* } }

impl S {
    #[rustc_abi(debug)]
    const C: () = (); // { dg-error "" "" { target *-*-* } }
}

impl S {
    #[rustc_abi(debug)]
    fn assoc_test(&self) { } // { dg-error "" "" { target *-*-* } }
}

#[rustc_abi(assert_eq)]
type TestAbiEq = (fn(bool), fn(bool));

#[rustc_abi(assert_eq)]
type TestAbiNe = (fn(u8), fn(u32)); // { dg-error "" "" { target *-*-* } }

#[rustc_abi(assert_eq)]
type TestAbiNeLarger = (fn([u8; 32]), fn([u32; 32])); // { dg-error "" "" { target *-*-* } }

#[rustc_abi(assert_eq)]
type TestAbiNeFloat = (fn(f32), fn(u32)); // { dg-error "" "" { target *-*-* } }

// Sign matters on some targets (such as s390x), so let's make sure we never accept this.
#[rustc_abi(assert_eq)]
type TestAbiNeSign = (fn(i32), fn(u32)); // { dg-error "" "" { target *-*-* } }

#[rustc_abi(assert_eq)]
type TestAbiEqNonsense = (fn((str, str)), fn((str, str))); // { dg-error ".E0277." "" { target *-*-* } }

