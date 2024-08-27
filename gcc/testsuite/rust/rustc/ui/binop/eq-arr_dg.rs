fn main() {
    struct X;
// { help "" "" { target *-*-* } .-1 }
    let xs = [X, X, X];
    let eq = xs == [X, X, X];
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

