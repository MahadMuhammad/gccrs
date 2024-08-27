fn main() {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
    |_: [_; return || {}]| {};
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }

    [(); return || {}];
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }

    [(); return |ice| {}];
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }

    [(); return while let Some(n) = Some(0) {}];
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }
}

