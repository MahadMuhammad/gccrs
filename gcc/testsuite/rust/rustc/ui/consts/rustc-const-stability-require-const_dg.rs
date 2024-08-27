#![crate_type = "lib"]
#![feature(staged_api)]
#![stable(feature = "foo", since = "1.0.0")]

#[stable(feature = "foo", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_foo", issue = "none")]
pub fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }

#[stable(feature = "bar", since = "1.0.0")]
#[rustc_const_stable(feature = "const_bar", since = "1.0.0")]
pub fn bar() {}
// { dg-error "" "" { target *-*-* } .-1 }

#[stable(feature = "potato", since = "1.0.0")]
pub struct Potato;

impl Potato {
    #[stable(feature = "salad", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_salad", issue = "none")]
    pub fn salad(&self) -> &'static str { "mmmmmm" }
// { dg-error "" "" { target *-*-* } .-1 }

    #[stable(feature = "roasted", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_roasted", issue = "none")]
    pub fn roasted(&self) -> &'static str { "mmmmmmmmmm" }
// { dg-error "" "" { target *-*-* } .-1 }
}

#[stable(feature = "bar", since = "1.0.0")]
#[rustc_const_stable(feature = "const_bar", since = "1.0.0")]
pub extern "C" fn bar_c() {}
// { dg-error "" "" { target *-*-* } .-1 }

#[stable(feature = "foo", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_foo", issue = "none")]
pub extern "C" fn foo_c() {}
// { dg-error "" "" { target *-*-* } .-1 }


#[stable(feature = "foobar", since = "1.0.0")]
#[rustc_const_unstable(feature = "foobar_const", issue = "none")]
pub const fn foobar() {}

#[stable(feature = "barfoo", since = "1.0.0")]
#[rustc_const_stable(feature = "barfoo_const", since = "1.0.0")]
pub const fn barfoo() {}

