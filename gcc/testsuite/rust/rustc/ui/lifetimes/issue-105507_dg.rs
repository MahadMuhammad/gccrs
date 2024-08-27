//@ run-rustfix
//
#![allow(warnings)]
struct Wrapper<'a, T: ?Sized>(&'a T);

trait Project {
    type Projected<'a> where Self: 'a;
    fn project(this: Wrapper<'_, Self>) -> Self::Projected<'_>;
}
trait MyTrait {}
trait ProjectedMyTrait {}

impl<T> Project for Option<T> {
    type Projected<'a> = Option<Wrapper<'a, T>> where T: 'a;
    fn project(this: Wrapper<'_, Self>) -> Self::Projected<'_> {
        this.0.as_ref().map(Wrapper)
    }
}

impl<T: MyTrait> MyTrait for Option<Wrapper<'_, T>> {}

impl<T: ProjectedMyTrait> MyTrait for Wrapper<'_, T> {}

impl<T> ProjectedMyTrait for T
    where
        T: Project,
        for<'a> T::Projected<'a>: MyTrait,
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
{}

fn require_trait<T: MyTrait>(_: T) {}

fn foo<T : MyTrait, U : MyTrait>(wrap: Wrapper<'_, Option<T>>, wrap1: Wrapper<'_, Option<U>>) {
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    require_trait(wrap);
// { dg-error "" "" { target *-*-* } .-1 }
    require_trait(wrap1);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

