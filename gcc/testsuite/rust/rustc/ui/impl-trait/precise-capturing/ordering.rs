fn lt<'a>() -> impl Sized + use<'a, 'a> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn ty<T>() -> impl Sized + use<T, T> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn ct<const N: usize>() -> impl Sized + use<N, N> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn ordering<'a, T>() -> impl Sized + use<T, 'a> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

