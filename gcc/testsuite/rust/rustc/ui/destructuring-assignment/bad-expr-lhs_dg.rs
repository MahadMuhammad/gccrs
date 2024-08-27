fn main() {
    1 = 2; // { dg-error ".E0070." "" { target *-*-* } }
    1 += 2; // { dg-error ".E0070." "" { target *-*-* } }
    (1, 2) = (3, 4);
// { dg-error ".E0070." "" { target *-*-* } .-1 }
// { dg-error ".E0070." "" { target *-*-* } .-2 }
}

