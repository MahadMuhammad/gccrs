struct S { x: u8, y: u8 }

fn main() {
    let (a, b) = (1, 2);

    (a, b) = (3, 4);
    (a, b) += (3, 4); // { dg-error ".E0067." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }

    [a, b] = [3, 4];
    [a, b] += [3, 4]; // { dg-error ".E0067." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }

    let s = S { x: 3, y: 4 };

    S { x: a, y: b } = s;
    S { x: a, y: b } += s; // { dg-error ".E0067." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }

    S { x: a, ..s } = S { x: 3, y: 4 };
// { dg-error "" "" { target *-*-* } .-1 }
}

