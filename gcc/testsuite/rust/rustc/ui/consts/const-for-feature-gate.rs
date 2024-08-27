// gate-test-const_for

const _: () = {
    for _ in 0..5 {}
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
// { dg-error ".E0015." "" { target *-*-* } .-3 }
// { dg-error ".E0015." "" { target *-*-* } .-4 }
};

fn main() {}

