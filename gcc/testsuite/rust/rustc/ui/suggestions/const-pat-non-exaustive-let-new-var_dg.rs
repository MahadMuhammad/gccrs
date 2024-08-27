fn main() {
    let A = 3;
// { dg-error ".E0005." "" { target *-*-* } .-1 }
// { dg-error ".E0005." "" { target *-*-* } .-2 }
// { dg-error ".E0005." "" { target *-*-* } .-3 }
// { help ".E0005." "" { target *-*-* } .-4 }
// { suggestion ".E0005." "" { target *-*-* } .-5 }

    const A: i32 = 2;
}

