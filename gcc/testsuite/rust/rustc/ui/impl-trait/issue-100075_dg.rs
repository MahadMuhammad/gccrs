trait Marker {}
impl<T> Marker for T {}

fn maybe<T>(
    _t: T,
) -> Option<
    //removing the line below makes it compile
    &'static T,
> {
    None
}

fn _g<T>(t: &'static T) -> &'static impl Marker {
// { dg-error ".E0720." "" { target *-*-* } .-1 }
    if let Some(t) = maybe(t) {
        return _g(t);
    }
    todo!()
}

fn main() {}

