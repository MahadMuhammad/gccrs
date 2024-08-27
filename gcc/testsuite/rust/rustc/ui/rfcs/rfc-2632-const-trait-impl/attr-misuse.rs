#![feature(const_trait_impl)]

#[const_trait]
trait A {
    #[const_trait] // { dg-error "" "" { target *-*-* } }
    fn foo(self);
}

#[const_trait] // { dg-error "" "" { target *-*-* } }
fn main() {}

