//@ revisions: old next
//@[next] compile-flags: -Znext-solver

#![feature(type_alias_impl_trait)]
trait Trait<'a> {
    type Out<U>;
}

impl<'a, T> Trait<'a> for T {
    type Out<U> = T;
}

type Foo = impl Sized;
// { dg-error "" "" { target *-*-* } .-1 }

fn weird_bound<X>(x: &<X as Trait<'static>>::Out<Foo>) -> X
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
where
    for<'a> X: Trait<'a>,
    for<'a> <X as Trait<'a>>::Out<()>: Copy,
{
    let x = *x; // { dg-error "" "" { target *-*-* } }
    todo!();
}

fn main() {
    let _: () = weird_bound(&());
}

