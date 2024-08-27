//@ normalize-stderr-test: "long-type-\d+" -> "long-type-hash"
trait Next {
    type Next: Next;
}

struct GetNext<T: Next> {
    t: T,
}

impl<T: Next> Next for GetNext<T> {
    type Next = <GetNext<T::Next> as Next>::Next;
// { dg-error ".E0275." "" { target *-*-* } .-1 }
}

fn main() {}

