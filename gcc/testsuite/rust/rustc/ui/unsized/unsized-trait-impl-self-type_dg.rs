// Test sized-ness checking in substitution in impls.

// impl - struct
trait T3<Z: ?Sized> {
    fn foo(&self, z: &Z);
}

struct S5<Y>(Y);

impl<X: ?Sized> T3<X> for S5<X> {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
}

fn main() { }

