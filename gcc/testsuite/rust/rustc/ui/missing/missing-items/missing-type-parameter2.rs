struct X<const N: u8>();

impl X<N> {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }
// { dg-error ".E0747." "" { target *-*-* } .-2 }
impl<T, const A: u8 = 2> X<N> {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }
// { dg-error ".E0747." "" { target *-*-* } .-2 }
// { dg-error ".E0747." "" { target *-*-* } .-3 }

fn foo(_: T) where T: Send {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }

fn bar<const N: u8>(_: A) {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }

fn main() {
}

