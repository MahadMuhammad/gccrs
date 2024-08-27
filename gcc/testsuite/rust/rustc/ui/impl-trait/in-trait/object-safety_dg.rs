use std::fmt::Debug;

trait Foo {
    fn baz(&self) -> impl Debug;
}

impl Foo for u32 {
    fn baz(&self) -> impl Debug {
        32
    }
}

fn main() {
    let i = Box::new(42_u32) as Box<dyn Foo>;
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
    let s = i.baz();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
}

