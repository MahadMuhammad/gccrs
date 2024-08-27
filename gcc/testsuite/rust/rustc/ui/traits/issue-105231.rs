// { dg-error "" "" { target *-*-* } }
struct A<T>(B<T>);
// { dg-error ".E0072." "" { target *-*-* } .-1 }
// { dg-error ".E0072." "" { target *-*-* } .-2 }
struct B<T>(A<A<T>>);
// { dg-error "" "" { target *-*-* } .-1 }
trait Foo {}
impl<T> Foo for T where T: Send {}
impl Foo for B<u8> {}

fn main() {}

