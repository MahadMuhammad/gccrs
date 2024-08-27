// Don't ICE when trying to annotate signature and we see `&()`

fn f<'a, T>(_: &'static &'a (), x: &'a T) -> &'static T {
    x
}
trait W<'a> {
    fn g<T>(self, x: &'a T) -> &'static T;
}

// Frankly this error message is impossible to parse, but :shrug:.
impl<'a> W<'a> for &'static () {
    fn g<T>(self, x: &'a T) -> &'static T {
        f(&self, x)
// { dg-error ".E0597." "" { target *-*-* } .-1 }
// { dg-error ".E0597." "" { target *-*-* } .-2 }
    }
}

fn main() {}

