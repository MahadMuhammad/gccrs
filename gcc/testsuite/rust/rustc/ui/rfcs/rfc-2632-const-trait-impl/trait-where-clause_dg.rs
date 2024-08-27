#![feature(const_trait_impl)]

#[const_trait]
trait Bar {}

trait Foo {
    fn a();
    fn b() where Self: ~const Bar;
// { dg-error "" "" { target *-*-* } .-1 }
    fn c<T: ~const Bar>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test1<T: Foo>() {
    T::a();
    T::b();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    T::c::<T>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn test2<T: Foo + Bar>() {
    T::a();
    T::b();
    T::c::<T>();
}

fn main() {}

