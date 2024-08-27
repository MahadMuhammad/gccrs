fn server() -> impl {
// { dg-error "" "" { target *-*-* } .-1 }
    ().map2(|| "")
}

trait FilterBase2 {
    fn map2<F>(self, f: F) -> Map2<F> {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

struct Map2<Segment2> {
    _func: F,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

impl<F> FilterBase2 for F {}

fn main() {}

