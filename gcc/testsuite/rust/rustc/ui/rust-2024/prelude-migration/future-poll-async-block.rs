//@ revisions: e2021 e2024
// { dg-additional-options "-frust-edition= 2021" }
//@[e2021] run-rustfix
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options
//@[e2024] check-pass

#![deny(rust_2024_prelude_collisions)]
trait Meow {
    fn poll(&self, _ctx: &mut core::task::Context<'_>) {}
}
impl<T> Meow for T {}
fn main() {
    core::pin::pin!(async {}).poll(&mut context());
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

fn context() -> core::task::Context<'static> {
    loop {}
}

