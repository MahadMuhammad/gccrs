#![feature(type_alias_impl_trait)]

fn main() {}

type WrongGeneric<T: 'static> = impl 'static;
// { dg-error "" "" { target *-*-* } .-1 }

fn wrong_generic<U: 'static, V: 'static>(_: U, v: V) -> WrongGeneric<U> {
    v
// { dg-error "" "" { target *-*-* } .-1 }
}

