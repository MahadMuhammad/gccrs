//@ ignore-compare-mode-next-solver (hangs)

#![feature(type_alias_impl_trait)]

type Bar<'a, 'b> = impl PartialEq<Bar<'b, 'static>> + std::fmt::Debug;

fn bar<'a, 'b>(i: &'a i32) -> Bar<'a, 'b> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

type Foo<'a, 'b> = impl PartialEq<Foo<'static, 'b>> + std::fmt::Debug;

fn foo<'a, 'b>(i: &'a i32) -> Foo<'a, 'b> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

type Moo<'a, 'b> = impl PartialEq<Moo<'static, 'a>> + std::fmt::Debug;

fn moo<'a, 'b>(i: &'a i32) -> Moo<'a, 'b> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {
    let meh = 42;
    let muh = 69;
    assert_eq!(bar(&meh), bar(&meh));
}

