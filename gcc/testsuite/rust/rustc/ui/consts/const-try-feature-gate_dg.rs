// gate-test-const_try

const fn t() -> Option<()> {
    Some(())?;
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
// { dg-error ".E0015." "" { target *-*-* } .-3 }
    None
}

fn main() {}

