#![crate_type="lib"]

struct A<const N: u8>;
trait Foo {}
impl Foo for A<N> {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }
// { dg-error ".E0747." "" { target *-*-* } .-2 }

struct B<const N: u8>;
impl<N> Foo for B<N> {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }

struct C<const C: u8, const N: u8>;
impl<const N: u8> Foo for C<N, T> {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }
// { dg-error ".E0747." "" { target *-*-* } .-2 }

