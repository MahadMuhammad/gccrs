//@ revisions: min
// we use a single revision because this should have a `full` revision
// but right now that ICEs and I(@BoxyUwU) could not get stderr normalization to work

// #![cfg_attr(full, feature(generic_const_exprs))]
// #![cfg_attr(full, allow(incomplete_features))]

const fn foo<T>() -> usize { std::mem::size_of::<T>() }
const fn bar<const N: usize>() -> usize { N }
const fn faz<'a>(_: &'a ()) -> usize { 13 }
const fn baz<'a>(_: &'a ()) -> usize where &'a (): Sized { 13 }

struct Foo<const N: usize>;
fn test<'a, 'b, T, const N: usize>() where &'b (): Sized {
    let _: [u8; foo::<T>()]; // { dg-error "" "" { target *-*-* } }
    let _: [u8; bar::<N>()]; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _: [u8; faz::<'a>(&())]; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _: [u8; baz::<'a>(&())]; // { dg-error "" "" { target *-*-* } }
    let _: [u8; faz::<'b>(&())]; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _: [u8; baz::<'b>(&())]; // { dg-error "" "" { target *-*-* } }

    let _ = [0; foo::<T>()]; // { dg-error "" "" { target *-*-* } }
    let _ = [0; bar::<N>()]; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = [0; faz::<'a>(&())]; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = [0; baz::<'a>(&())]; // { dg-error "" "" { target *-*-* } }
    let _ = [0; faz::<'b>(&())]; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = [0; baz::<'b>(&())]; // { dg-error "" "" { target *-*-* } }
    let _: Foo<{ foo::<T>() }>; // { dg-error "" "" { target *-*-* } }
    let _: Foo<{ bar::<N>() }>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _: Foo<{ faz::<'a>(&()) }>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _: Foo<{ baz::<'a>(&()) }>; // { dg-error "" "" { target *-*-* } }
    let _: Foo<{ faz::<'b>(&()) }>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _: Foo<{ baz::<'b>(&()) }>; // { dg-error "" "" { target *-*-* } }
    let _ = Foo::<{ foo::<T>() }>; // { dg-error "" "" { target *-*-* } }
    let _ = Foo::<{ bar::<N>() }>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = Foo::<{ faz::<'a>(&()) }>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = Foo::<{ baz::<'a>(&()) }>; // { dg-error "" "" { target *-*-* } }
    let _ = Foo::<{ faz::<'b>(&()) }>; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = Foo::<{ baz::<'b>(&()) }>; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

