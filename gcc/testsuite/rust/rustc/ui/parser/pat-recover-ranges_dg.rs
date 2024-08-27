fn main() {
    match -1 {
        0..=1 => (),
        0..=(1) => (),
// { dg-error "" "" { target *-*-* } .-1 }
        (-12)..=4 => (),
// { dg-error "" "" { target *-*-* } .-1 }
        (0)..=(-4) => (),
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        ..=1 + 2 => (),
// { dg-error "" "" { target *-*-* } .-1 }
        (4).. => (),
// { dg-error "" "" { target *-*-* } .-1 }
        (-4 + 0).. => (),
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        (1 + 4)...1 * 2 => (),
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-warning "" "" { target *-*-* } .-5 }
        0.x()..="y".z() => (),
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    };
}

macro_rules! m {
    ($pat:pat) => {};
    (($s:literal)..($e:literal)) => {};
}

m!((7)..(7));

