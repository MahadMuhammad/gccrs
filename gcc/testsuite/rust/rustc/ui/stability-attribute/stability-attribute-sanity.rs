// Various checks that stability attributes are used correctly, per RFC 507

#![feature(staged_api)]

#![stable(feature = "rust1", since = "1.0.0")]

mod bogus_attribute_types_1 {
    #[stable(feature = "a", since = "4.4.4", reason)] // { dg-error ".E0541." "" { target *-*-* } }
    fn f1() { }

    #[stable(feature = "a", since)] // { dg-error ".E0539." "" { target *-*-* } }
    fn f2() { }

    #[stable(feature, since = "3.3.3")] // { dg-error ".E0539." "" { target *-*-* } }
    fn f3() { }

    #[stable(feature = "a", since(b))] // { dg-error ".E0539." "" { target *-*-* } }
    fn f5() { }

    #[stable(feature(b), since = "3.3.3")] // { dg-error ".E0539." "" { target *-*-* } }
    fn f6() { }
}

mod missing_feature_names {
    #[unstable(issue = "none")] // { dg-error ".E0546." "" { target *-*-* } }
    fn f1() { }

    #[unstable(feature = "b")] // { dg-error ".E0547." "" { target *-*-* } }
    fn f2() { }

    #[stable(since = "3.3.3")] // { dg-error ".E0546." "" { target *-*-* } }
    fn f3() { }
}

mod missing_version {
    #[stable(feature = "a")] // { dg-error ".E0542." "" { target *-*-* } }
    fn f1() { }

    #[stable(feature = "a", since = "4.4.4")]
    #[deprecated(note = "a")] // { dg-error ".E0542." "" { target *-*-* } }
    fn f2() { }

    #[stable(feature = "a", since = "4.4.4")]
    #[deprecated(since = "5.5.5")] // { dg-error ".E0543." "" { target *-*-* } }
    fn f3() { }
}

#[unstable(feature = "b", issue = "none")]
#[stable(feature = "a", since = "4.4.4")] // { dg-error ".E0544." "" { target *-*-* } }
fn multiple1() { }

#[unstable(feature = "b", issue = "none")]
#[unstable(feature = "b", issue = "none")] // { dg-error ".E0544." "" { target *-*-* } }
fn multiple2() { }

#[stable(feature = "a", since = "4.4.4")]
#[stable(feature = "a", since = "4.4.4")] // { dg-error ".E0544." "" { target *-*-* } }
fn multiple3() { }

#[stable(feature = "e", since = "b")] // { dg-error "" "" { target *-*-* } }
#[deprecated(since = "5.5.5", note = "text")]
#[deprecated(since = "5.5.5", note = "text")] // { dg-error "" "" { target *-*-* } }
#[rustc_const_unstable(feature = "c", issue = "none")]
#[rustc_const_unstable(feature = "d", issue = "none")] // { dg-error ".E0544." "" { target *-*-* } }
pub const fn multiple4() { }

#[stable(feature = "a", since = "1.0.0")] // { dg-error ".E0711." "" { target *-*-* } }
#[deprecated(since = "invalid", note = "text")] // { dg-error "" "" { target *-*-* } }
fn invalid_deprecation_version() {}

#[deprecated(since = "5.5.5", note = "text")]
fn deprecated_without_unstable_or_stable() { }
// { dg-error ".E0549." "" { target *-*-* } .-2 }

fn main() { }

