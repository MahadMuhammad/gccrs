fn main() {
    let xs = [13, 1, 5, 2, 3, 1, 21, 8];
    let [a @ 3.., b @ ..3, c @ 4..6, ..] = xs;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
// { dg-error ".E0005." "" { target *-*-* } .-2 }
}

