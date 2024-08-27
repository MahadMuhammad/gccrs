//@ run-rustfix

struct GenericAssocMethod<T>(T);

impl<T> GenericAssocMethod<T> {
    fn default_hello() {}
    fn self_ty_hello(_: Self) {}
    fn self_ty_ref_hello(_: &Self) {}
}

fn main() {
    // Test for inferred types
    let x = GenericAssocMethod(33);
    x.self_ty_ref_hello();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    x.self_ty_hello();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    // Test for known types
    let y = GenericAssocMethod(33i32);
    y.default_hello();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    y.self_ty_ref_hello();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    y.self_ty_hello();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

