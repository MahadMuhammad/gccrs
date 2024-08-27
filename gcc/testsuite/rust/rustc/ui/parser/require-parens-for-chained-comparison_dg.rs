fn main() {
    false == false == false;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    false == 0 < 2;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    f<X>();
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    f<Result<Option<X>, Option<Option<X>>>(1, 2);
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    let _ = f<u8, i8>();
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    let _ = f<'_, i8>();
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }

    f<'_>();
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }

    let _ = f<u8>;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

