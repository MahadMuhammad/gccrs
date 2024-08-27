#![feature(type_alias_impl_trait)]

fn main() {}

trait T {}
impl T for i32 {}

fn should_ret_unit() -> impl T {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    panic!()
}

type Foo = impl T;

fn a() -> Foo {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    panic!()
}

fn b() -> Foo {
    42
}

