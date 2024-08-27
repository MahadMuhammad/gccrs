const TEST4: fn() -> _ = 42;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-error ".E0121." "" { target *-*-* } .-2 }

fn main() {
    const TEST5: fn() -> _ = 42;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-error ".E0121." "" { target *-*-* } .-2 }
}

