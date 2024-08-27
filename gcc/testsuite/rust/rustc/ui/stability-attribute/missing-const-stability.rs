//@ compile-flags: -Znext-solver
#![feature(staged_api)]
#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }
#![stable(feature = "stable", since = "1.0.0")]

#[stable(feature = "stable", since = "1.0.0")]
pub const fn foo() {} // { dg-error "" "" { target *-*-* } }

#[unstable(feature = "unstable", issue = "none")]
pub const fn bar() {} // ok because function is unstable

#[stable(feature = "stable", since = "1.0.0")]
pub struct Foo;
impl Foo {
    #[stable(feature = "stable", since = "1.0.0")]
    pub const fn foo() {} // { dg-error "" "" { target *-*-* } }

    #[unstable(feature = "unstable", issue = "none")]
    pub const fn bar() {} // ok because function is unstable
}

#[stable(feature = "stable", since = "1.0.0")]
#[const_trait]
pub trait Bar {
    #[stable(feature = "stable", since = "1.0.0")]
    fn fun();
}
#[stable(feature = "stable", since = "1.0.0")]
impl const Bar for Foo {
// { dg-error "" "" { target *-*-* } .-1 }
    fn fun() {}
}

fn main() {}

