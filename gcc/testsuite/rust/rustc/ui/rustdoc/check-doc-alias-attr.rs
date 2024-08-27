#![crate_type = "lib"]

#[doc(alias = "foo")] // ok!
#[doc(alias("bar", "baz"))] // ok!
pub struct Bar;

#[doc(alias)] // { dg-error "" "" { target *-*-* } }
#[doc(alias = 0)] // { dg-error "" "" { target *-*-* } }
#[doc(alias = "\"")] // { dg-error "" "" { target *-*-* } }
#[doc(alias = "\n")] // { dg-error "" "" { target *-*-* } }
#[doc(alias = "
// { dg-error "" "" { target *-*-* } .-1 }
#[doc(alias = "\t")] // { dg-error "" "" { target *-*-* } }
#[doc(alias = " hello")] // { dg-error "" "" { target *-*-* } }
#[doc(alias = "hello ")] // { dg-error "" "" { target *-*-* } }
#[doc(alias = "")] // { dg-error "" "" { target *-*-* } }
pub struct Foo;

#[doc(alias(0))] // { dg-error "" "" { target *-*-* } }
#[doc(alias("\""))] // { dg-error "" "" { target *-*-* } }
#[doc(alias("\n"))] // { dg-error "" "" { target *-*-* } }
#[doc(alias("
// { dg-error "" "" { target *-*-* } .-1 }
#[doc(alias("\t"))] // { dg-error "" "" { target *-*-* } }
#[doc(alias(" hello"))] // { dg-error "" "" { target *-*-* } }
#[doc(alias("hello "))] // { dg-error "" "" { target *-*-* } }
#[doc(alias(""))] // { dg-error "" "" { target *-*-* } }
pub struct Foo2;

