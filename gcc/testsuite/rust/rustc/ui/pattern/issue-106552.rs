fn main() {
    let 5 = 6;
// { dg-error ".E0005." "" { target *-*-* } .-1 }

    let x @ 5 = 6;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
}

