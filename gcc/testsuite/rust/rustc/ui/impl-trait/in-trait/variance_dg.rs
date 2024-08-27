#![feature(rustc_attrs)]
#![allow(internal_features)]
#![rustc_variance_of_opaques]

trait Captures<'a> {}
impl<T> Captures<'_> for T {}

trait Foo<'i> {
    fn implicit_capture_early<'a: 'a>() -> impl Sized {}
// { dg-error "" "" { target *-*-* } .-1 }
    // Self, 'i, 'a, 'i_duplicated, 'a_duplicated

    fn explicit_capture_early<'a: 'a>() -> impl Sized + Captures<'a> {} // { dg-error "" "" { target *-*-* } }

    fn implicit_capture_late<'a>(_: &'a ()) -> impl Sized {} // { dg-error "" "" { target *-*-* } }

    fn explicit_capture_late<'a>(_: &'a ()) -> impl Sized + Captures<'a> {} // { dg-error "" "" { target *-*-* } }
}

fn main() {}

