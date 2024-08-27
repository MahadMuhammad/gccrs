use std::ops::AddAssign;
use std::mem::ManuallyDrop;

struct NonCopy;
impl AddAssign for NonCopy {
    fn add_assign(&mut self, _: Self) {}
}

union Foo {
    a: u8, // non-dropping
    b: ManuallyDrop<NonCopy>,
}

fn main() {
    let mut foo = Foo { a: 42 };
    foo.a += 5; // { dg-error ".E0133." "" { target *-*-* } }
    *foo.b += NonCopy; // { dg-error ".E0133." "" { target *-*-* } }
    *foo.b = NonCopy; // { dg-error ".E0133." "" { target *-*-* } }
    foo.b = ManuallyDrop::new(NonCopy);
    foo.a; // { dg-error ".E0133." "" { target *-*-* } }
    let foo = Foo { a: 42 };
    foo.b; // { dg-error ".E0133." "" { target *-*-* } }
    let mut foo = Foo { a: 42 };
    foo.b = foo.b;
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

