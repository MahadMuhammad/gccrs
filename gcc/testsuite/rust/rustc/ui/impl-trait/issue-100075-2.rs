fn opaque<T>(t: T) -> impl Sized {
// { dg-error ".E0720." "" { target *-*-* } .-1 }
// { dg-warning ".E0720." "" { target *-*-* } .-2 }
    opaque(Some(t))
}

#[allow(dead_code)]
fn main() {}

