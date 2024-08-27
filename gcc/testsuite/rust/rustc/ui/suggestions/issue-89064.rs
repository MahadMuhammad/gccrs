use std::convert::TryInto;

trait A<T> {
    fn foo() {}
}

trait B<T, U> {
    fn bar() {}
}

struct S;

impl<T> A<T> for S {}
impl<T, U> B<T, U> for S {}

fn main() {
    let _ = A::foo::<S>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }

    let _ = B::bar::<S, S>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }

    let _ = A::<S>::foo::<S>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    let _ = 42.into::<Option<_>>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
}

