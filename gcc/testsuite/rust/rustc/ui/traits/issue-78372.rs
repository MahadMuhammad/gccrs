use std::ops::DispatchFromDyn; // { dg-error ".E0658." "" { target *-*-* } }
struct Smaht<T, MISC>(PhantomData); // { dg-error ".E0412." "" { target *-*-* } }
impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {} // { dg-error ".E0378." "" { target *-*-* } }
// { dg-error ".E0378." "" { target *-*-* } .-1 }
// { dg-error ".E0378." "" { target *-*-* } .-2 }
// { dg-error ".E0378." "" { target *-*-* } .-3 }
trait Foo: X<u32> {}
trait X<T> {
    fn foo(self: Smaht<Self, T>); // { dg-error ".E0307." "" { target *-*-* } }
}
trait Marker {}
impl Marker for dyn Foo {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
fn main() {}

