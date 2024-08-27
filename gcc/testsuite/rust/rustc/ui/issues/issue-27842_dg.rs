fn main() {
    let tup = (0, 1, 2);
    // the case where we show a suggestion
    let _ = tup[0];
// { dg-error ".E0608." "" { target *-*-* } .-1 }

    // the case where we show just a general hint
    let i = 0_usize;
    let _ = tup[i];
// { dg-error ".E0608." "" { target *-*-* } .-1 }

    // the case where the index is out of bounds
    let tup = (10,);
    let _ = tup[3];
// { dg-error ".E0608." "" { target *-*-* } .-1 }
}

