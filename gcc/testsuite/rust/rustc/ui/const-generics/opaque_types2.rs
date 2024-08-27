#![feature(type_alias_impl_trait)]

type Foo = impl Sized;

fn foo<const C: u32>() {}

const C: Foo = 42;

fn bar()
where
    Foo:,
{
    foo::<C>();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

