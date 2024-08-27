// This test case checks the behavior of typeck::check::method::suggest::is_fn on Ty::Error.

struct Foo;

trait Bar {
// { dg-note "" "" { target *-*-* } .-1 }
    fn bar(&self) {}
}

impl Bar for Foo {}

fn main() {
    let arc = std::sync::Arc::new(oops);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-note ".E0425." "" { target *-*-* } .-2 }
    arc.bar();

    let arc2 = std::sync::Arc::new(|| Foo);
    arc2.bar();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }
// { help ".E0599." "" { target *-*-* } .-3 }
// { help ".E0599." "" { target *-*-* } .-4 }
}

