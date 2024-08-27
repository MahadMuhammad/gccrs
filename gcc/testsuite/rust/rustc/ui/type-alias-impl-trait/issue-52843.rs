#![feature(type_alias_impl_trait)]

type Foo<T> = impl Default;

#[allow(unused)]
fn foo<T: Default>(t: T) -> Foo<T> {
    t
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct NotDefault;

fn main() {
    let _ = Foo::<NotDefault>::default();
}

