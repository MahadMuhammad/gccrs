fn f(a: isize, b: isize) : lt(a, b) { }
// { dg-error ".E0573." "" { target *-*-* } .-1 }
// { dg-error ".E0573." "" { target *-*-* } .-2 }
// { dg-error ".E0573." "" { target *-*-* } .-3 }
// { dg-error ".E0573." "" { target *-*-* } .-4 }

fn lt(a: isize, b: isize) { }

fn main() {
    let a: isize = 10;
    let b: isize = 23;
    check (lt(a, b));
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    f(a, b);
}

