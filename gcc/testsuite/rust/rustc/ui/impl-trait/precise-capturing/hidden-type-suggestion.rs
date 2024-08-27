fn lifetime<'a, 'b>(x: &'a ()) -> impl Sized + use<'b> {
// { help "" "" { target *-*-* } .-1 }
    x
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn param<'a, T>(x: &'a ()) -> impl Sized + use<T> {
// { help "" "" { target *-*-* } .-1 }
    x
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn empty<'a>(x: &'a ()) -> impl Sized + use<> {
// { help "" "" { target *-*-* } .-1 }
    x
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

trait Captures<'a> {}
impl<T> Captures<'_> for T {}

fn missing<'a, 'captured, 'not_captured, Captured>(x: &'a ()) -> impl Captures<'captured> {
// { help "" "" { target *-*-* } .-1 }
    x
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn no_params_yet(_: impl Sized, y: &()) -> impl Sized {
// { help "" "" { target *-*-* } .-1 }
    y
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn yes_params_yet<'a, T>(_: impl Sized, y: &'a ()) -> impl Sized {
// { help "" "" { target *-*-* } .-1 }
    y
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn main() {}

