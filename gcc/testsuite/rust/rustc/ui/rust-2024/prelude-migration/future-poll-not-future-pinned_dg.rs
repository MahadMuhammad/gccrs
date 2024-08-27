//@ revisions: e2021 e2024
// { dg-additional-options "-frust-edition= 2021" }
//@[e2021] run-rustfix
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options
//@[e2024] check-pass

#![deny(rust_2024_prelude_collisions)]
trait Meow {
    fn poll(&self) {}
}
impl<T> Meow for T {}
fn main() {
    // This is a deliberate false positive.
    // While `()` does not implement `Future` and can therefore not be ambiguous, we
    // do not check that in the lint, as that introduces additional complexities.
    // Just checking whether the self type is `Pin<&mut _>` is enough.
    core::pin::pin!(()).poll();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

