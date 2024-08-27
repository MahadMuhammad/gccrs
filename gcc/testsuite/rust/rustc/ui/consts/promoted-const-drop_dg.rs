#![feature(const_trait_impl)]
#![feature(const_mut_refs)]

struct A();

impl const Drop for A {
// { dg-error "" "" { target *-*-* } .-1 }
    fn drop(&mut self) {}
}

const C: A = A();

fn main() {
    let _: &'static A = &A(); // { dg-error ".E0716." "" { target *-*-* } }
    let _: &'static [A] = &[C]; // { dg-error ".E0716." "" { target *-*-* } }
}

