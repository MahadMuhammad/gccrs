// issue: 113314

#![feature(type_alias_impl_trait)]

type Op = impl std::fmt::Display;
fn foo() -> Op { &"hello world" }

fn transform<S>() -> impl std::fmt::Display {
    &0usize
}
fn bad() -> Op {
    transform::<Op>()
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    let mut x = foo();
    println!("{x}");
    x = bad();
    println!("{x}");
}

