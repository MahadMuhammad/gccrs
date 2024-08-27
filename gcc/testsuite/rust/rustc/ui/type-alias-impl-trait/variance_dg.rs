#![feature(rustc_attrs, type_alias_impl_trait, impl_trait_in_assoc_type)]
#![allow(internal_features)]
#![rustc_variance_of_opaques]

trait Captures<'a> {}
impl<T> Captures<'_> for T {}

type NotCapturedEarly<'a> = impl Sized; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

type CapturedEarly<'a> = impl Sized + Captures<'a>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

type NotCapturedLate<'a> = dyn for<'b> Iterator<Item = impl Sized>; // { dg-error ".E0657." "" { target *-*-* } }
// { dg-error ".E0657." "" { target *-*-* } .-1 }
// { dg-error ".E0657." "" { target *-*-* } .-2 }

type Captured<'a> = dyn for<'b> Iterator<Item = impl Sized + Captures<'a>>; // { dg-error ".E0657." "" { target *-*-* } }
// { dg-error ".E0657." "" { target *-*-* } .-1 }
// { dg-error ".E0657." "" { target *-*-* } .-2 }

type Bar<'a, 'b: 'b, T> = impl Sized; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

trait Foo<'i> {
    type ImplicitCapture<'a>;

    type ExplicitCaptureFromHeader<'a>;

    type ExplicitCaptureFromGat<'a>;
}

impl<'i> Foo<'i> for &'i () {
    type ImplicitCapture<'a> = impl Sized; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

    type ExplicitCaptureFromHeader<'a> = impl Sized + Captures<'i>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

    type ExplicitCaptureFromGat<'a> = impl Sized + Captures<'a>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
}

impl<'i> Foo<'i> for () {
    type ImplicitCapture<'a> = impl Sized; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

    type ExplicitCaptureFromHeader<'a> = impl Sized + Captures<'i>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }

    type ExplicitCaptureFromGat<'a> = impl Sized + Captures<'a>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
}

trait Nesting<'a> {
    type Output;
}
impl<'a> Nesting<'a> for &'a () {
    type Output = &'a ();
}
type NestedDeeply<'a> =
    impl Nesting< // { dg-error "" "" { target *-*-* } }
        'a,
        Output = impl Nesting< // { dg-error "" "" { target *-*-* } }
            'a,
            Output = impl Nesting< // { dg-error "" "" { target *-*-* } }
                'a,
                Output = impl Nesting< // { dg-error "" "" { target *-*-* } }
                    'a,
                    Output = impl Nesting<'a> // { dg-error "" "" { target *-*-* } }
                >
            >,
        >,
    >;
fn test<'a>() -> NestedDeeply<'a> {
    &()
}

fn main() {}

