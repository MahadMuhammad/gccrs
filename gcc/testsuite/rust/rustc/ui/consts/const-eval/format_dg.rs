const fn failure() {
    panic!("{:?}", 0);
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
}

const fn print() {
    println!("{:?}", 0);
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
// { dg-error ".E0015." "" { target *-*-* } .-3 }
}

fn main() {}

