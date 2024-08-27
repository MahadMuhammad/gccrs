#![feature(rustc_attrs)]

#[rustc_object_lifetime_default]
struct A<
    T, // { dg-error "" "" { target *-*-* } }
>(T);

#[rustc_object_lifetime_default]
struct B<
    'a,
    T, // { dg-error "" "" { target *-*-* } }
>(&'a (), T);

#[rustc_object_lifetime_default]
struct C<
    'a,
    T: 'a, // { dg-error "" "" { target *-*-* } }
>(&'a T);

#[rustc_object_lifetime_default]
struct D<
    'a,
    'b,
    T: 'a + 'b, // { dg-error "" "" { target *-*-* } }
>(&'a T, &'b T);

#[rustc_object_lifetime_default]
struct E<
    'a,
    'b: 'a,
    T: 'b, // { dg-error "" "" { target *-*-* } }
>(&'a T, &'b T);

#[rustc_object_lifetime_default]
struct F<
    'a,
    'b,
    T: 'a, // { dg-error "" "" { target *-*-* } }
    U: 'b, // { dg-error "" "" { target *-*-* } }
>(&'a T, &'b U);

#[rustc_object_lifetime_default]
struct G<
    'a,
    'b,
    T: 'a,      // { dg-error "" "" { target *-*-* } }
    U: 'a + 'b, // { dg-error "" "" { target *-*-* } }
>(&'a T, &'b U);

fn main() {}

