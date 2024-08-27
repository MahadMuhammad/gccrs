trait Provider {
    type A<'a>;
}

impl Provider for () {
    type A<'a> = ();
}

struct Holder<B> {
  inner: Box<dyn Provider<A = B>>,
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
}

fn main() {
    Holder {
        inner: Box::new(()), // { dg-error ".E0038." "" { target *-*-* } }
    };
}

