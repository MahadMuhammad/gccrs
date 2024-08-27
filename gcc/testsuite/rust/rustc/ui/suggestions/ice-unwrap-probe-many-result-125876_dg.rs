// Regression test for ICE #125876

fn main() {
    std::ptr::from_ref(num).cast_mut().as_deref();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
// { dg-warning ".E0599." "" { target *-*-* } .-3 }
// { dg-warning ".E0599." "" { target *-*-* } .-4 }
// { dg-warning ".E0599." "" { target *-*-* } .-5 }
// { dg-warning ".E0599." "" { target *-*-* } .-6 }
}

