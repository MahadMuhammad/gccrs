#![allow(bare_trait_objects)]
trait NotObjectSafe {
    fn foo() -> Self;
}

struct A;
struct B;

impl NotObjectSafe for A {
    fn foo() -> Self {
        A
    }
}

impl NotObjectSafe for B {
    fn foo() -> Self {
        B
    }
}

fn car() -> dyn NotObjectSafe { // { dg-error ".E0038." "" { target *-*-* } }
// { dg-error ".E0746." "" { target *-*-* } .-1 }
    if true {
        return A;
    }
    B
}

fn cat() -> Box<dyn NotObjectSafe> { // { dg-error ".E0038." "" { target *-*-* } }
    if true {
        return Box::new(A); // { dg-error ".E0038." "" { target *-*-* } }
    }
    Box::new(B) // { dg-error ".E0038." "" { target *-*-* } }
}

fn main() {}

