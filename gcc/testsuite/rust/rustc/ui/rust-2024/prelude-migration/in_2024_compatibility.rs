// { dg-additional-options "-frust-edition= 2021" }

#![deny(rust_2024_compatibility)]

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

