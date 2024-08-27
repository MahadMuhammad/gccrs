#![feature(type_alias_impl_trait)]

type Foo = impl Sized;
// { dg-error ".E0391." "" { target *-*-* } .-1 }
// { dg-error ".E0391." "" { target *-*-* } .-2 }

fn foo<const C: Foo>() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {
    foo::<42>();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

