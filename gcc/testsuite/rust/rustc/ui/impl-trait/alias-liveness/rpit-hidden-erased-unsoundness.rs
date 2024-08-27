// This test should never pass!

#![feature(type_alias_impl_trait)]

trait Captures<'a> {}
impl<T> Captures<'_> for T {}

struct MyTy<'a, 'b>(Option<*mut &'a &'b ()>);
unsafe impl Send for MyTy<'_, 'static> {}

fn step1<'a, 'b: 'a>() -> impl Sized + Captures<'b> + 'a {
    MyTy::<'a, 'b>(None)
}

fn step2<'a, 'b: 'a>() -> impl Sized + 'a {
    step1::<'a, 'b>()
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn step3<'a, 'b: 'a>() -> impl Send + 'a {
    step2::<'a, 'b>()
    // This should not be Send unless `'b: 'static`
}

fn main() {}

