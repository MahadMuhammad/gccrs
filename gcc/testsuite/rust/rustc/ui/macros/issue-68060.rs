fn main() {
    (0..)
        .map(
            #[target_feature(enable = "")]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
            #[track_caller]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
            |_| (),
// { dg-note "" "" { target *-*-* } .-1 }
        )
        .next();
}

