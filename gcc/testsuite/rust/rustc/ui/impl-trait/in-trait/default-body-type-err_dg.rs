use std::ops::Deref;

pub trait Foo {
    fn lol(&self) -> impl Deref<Target = String> {
// { dg-error ".E0271." "" { target *-*-* } .-1 }
        &1i32
    }
}

fn main() {}

