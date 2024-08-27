fn main() {
    let Some(1) = loop {
// { dg-note ".E0005." "" { target *-*-* } .-1 }
// { dg-error ".E0005." "" { target *-*-* } .-2 }
// { dg-note ".E0005." "" { target *-*-* } .-3 }
// { dg-note ".E0005." "" { target *-*-* } .-4 }
// { dg-note ".E0005." "" { target *-*-* } .-5 }
// { dg-note ".E0005." "" { target *-*-* } .-6 }
        break Some(1)
    } else {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        return;
    };
}

