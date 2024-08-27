fn no_elided_lt() -> impl Sized + use<'_> {}
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-error ".E0106." "" { target *-*-* } .-2 }

fn static_lt() -> impl Sized + use<'static> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn missing_lt() -> impl Sized + use<'missing> {}
// { dg-error ".E0261." "" { target *-*-* } .-1 }

fn main() {}

