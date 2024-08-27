//@ revisions: e2021 e2024
// { dg-additional-options "-frust-edition= 2021" }
//@[e2021] run-rustfix
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options
//@[e2024] check-pass

#![deny(rust_2024_prelude_collisions)]
trait Meow {
    fn into_future(&self) {}
}
impl Meow for Cat {}

struct Cat;

impl core::future::IntoFuture for Cat {
    type Output = ();
    type IntoFuture = core::future::Ready<()>;

    fn into_future(self) -> Self::IntoFuture {
        core::future::ready(())
    }
}

fn main() {
    Cat.into_future();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

