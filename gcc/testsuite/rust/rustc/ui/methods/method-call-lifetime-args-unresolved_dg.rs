fn main() {
    0.clone::<'a>();
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-warning ".E0261." "" { target *-*-* } .-2 }
// { dg-warning ".E0261." "" { target *-*-* } .-3 }
}

