fn main() {
    match u8::MAX {
        u8::MAX.abs() => (),
// { dg-error "" "" { target *-*-* } .-1 }
        x.sqrt() @ .. => (),
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        z @ w @ v.u() => (),
// { dg-error "" "" { target *-*-* } .-1 }
        y.ilog(3) => (),
// { dg-error "" "" { target *-*-* } .-1 }
        n + 1 => (),
// { dg-error "" "" { target *-*-* } .-1 }
        ("".f() + 14 * 8) => (),
// { dg-error "" "" { target *-*-* } .-1 }
        0 | ((1) | 2) | 3 => (),
        f?() => (),
// { dg-error "" "" { target *-*-* } .-1 }
        (_ + 1) => (),
// { dg-error "" "" { target *-*-* } .-1 }
    }

    let 1 + 1 = 2;
// { dg-error "" "" { target *-*-* } .-1 }

    let b = matches!(x, (x * x | x.f()) | x[0]);
// { dg-error "" "" { target *-*-* } .-1 }
}

