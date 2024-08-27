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

fn main() {
    // This is a false positive, but it should be rare enough to not matter, and checking whether
    // it implements the trait can have other nontrivial consequences, so it was decided to accept
    // this.
    Cat.into_future();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

