fn main() {
    let (mut a, mut b);
    [a, .., b, ..] = [0, 1]; // { dg-error "" "" { target *-*-* } }
    [a, a, b] = [1, 2];
// { dg-error ".E0527." "" { target *-*-* } .-1 }
    [_] = [1, 2];
// { dg-error ".E0527." "" { target *-*-* } .-1 }
}

