fn main() {
    println!("%100000", 1);
// { dg-error "" "" { target *-*-* } .-1 }
    println!("%     65536", 1);
// { dg-error "" "" { target *-*-* } .-1 }
}

