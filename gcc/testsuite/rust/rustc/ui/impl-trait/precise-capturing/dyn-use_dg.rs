fn dyn() -> &'static dyn use<> { &() }
// { dg-error "" "" { target *-*-* } .-1 }

