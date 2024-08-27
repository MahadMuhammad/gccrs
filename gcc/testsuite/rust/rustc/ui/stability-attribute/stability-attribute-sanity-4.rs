// Various checks that stability attributes are used correctly, per RFC 507

#![feature(staged_api)]

#![stable(feature = "rust1", since = "1.0.0")]

mod bogus_attribute_types_2 {
    #[unstable] // { dg-error "" "" { target *-*-* } }
    fn f1() { }

    #[unstable = "b"] // { dg-error "" "" { target *-*-* } }
    fn f2() { }

    #[stable] // { dg-error "" "" { target *-*-* } }
    fn f3() { }

    #[stable = "a"] // { dg-error "" "" { target *-*-* } }
    fn f4() { }

    #[stable(feature = "a", since = "3.3.3")]
    #[deprecated] // { dg-error ".E0542." "" { target *-*-* } }
// { dg-error ".E0543." "" { target *-*-* } .-1 }
    fn f5() { }

    #[stable(feature = "a", since = "3.3.3")]
    #[deprecated = "a"] // { dg-error ".E0542." "" { target *-*-* } }
    fn f6() { }
}

fn main() { }

