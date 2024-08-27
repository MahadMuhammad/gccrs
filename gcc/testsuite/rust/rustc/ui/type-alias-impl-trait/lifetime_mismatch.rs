#![feature(type_alias_impl_trait)]

type Foo<'a> = impl Sized;

fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> (Foo<'a>, Foo<'b>) {
    (x, y)
// { dg-error "" "" { target *-*-* } .-1 }
}

type Bar<'a, 'b> = impl std::fmt::Debug;

fn bar<'x, 'y>(i: &'x i32, j: &'y i32) -> (Bar<'x, 'y>, Bar<'y, 'x>) {
    (i, j)
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    let meh = 42;
    let muh = 69;
    println!("{:?}", bar(&meh, &muh));
}

