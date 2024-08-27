//@ revisions: e2018 e2021
// { dg-additional-options "-frust-edition=2018" }
// { dg-additional-options "-frust-edition=2021" }
//@[e2018] check-pass
#![crate_type = "lib"]
#![stable(feature = "foo", since = "1.0.0")]
#![feature(staged_api)]

#[stable(feature = "foo", since = "1.0.0")]
#[rustc_const_stable(feature = "foo", since = "1.0.0")]
const fn foo() {
    assert!(false);
    assert!(false, "foo");
    panic!({ "foo" });
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

