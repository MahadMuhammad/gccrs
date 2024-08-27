#![feature(const_trait_impl)]

#[const_trait]
trait Foo {}

const impl Foo for i32 {} // { dg-error "" "" { target *-*-* } }

#[const_trait]
trait Bar {}

const impl<T: Foo> Bar for T {} // { dg-error "" "" { target *-*-* } }

const fn still_implements<T: Bar>() {}

const _: () = still_implements::<i32>();

fn main() {}

