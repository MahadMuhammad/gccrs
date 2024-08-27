//@ run-rustfix
pub trait MyTrait {
    type T;

    fn bar(self) -> Self::T;
}

pub fn foo<A: MyTrait, B>(a: A) -> B {
    return a.bar(); // { dg-error ".E0308." "" { target *-*-* } }
}
fn main() {}

