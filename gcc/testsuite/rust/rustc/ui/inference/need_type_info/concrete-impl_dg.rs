trait Ambiguous<A> {
    fn method() {}
}

struct One;
struct Two;
struct Struct;

impl Ambiguous<One> for Struct {}
// { dg-note "" "" { target *-*-* } .-1 }
impl Ambiguous<Two> for Struct {}

fn main() {
    <Struct as Ambiguous<_>>::method();
// { dg-error ".E0283." "" { target *-*-* } .-1 }
// { dg-note ".E0283." "" { target *-*-* } .-2 }
}

