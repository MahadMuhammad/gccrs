union U {
    a: str,
// { dg-error ".E0740." "" { target *-*-* } .-1 }
// { dg-error ".E0740." "" { target *-*-* } .-2 }

    b: u8,
}

union W {
    a: u8,
    b: str,
// { dg-error ".E0740." "" { target *-*-* } .-1 }
// { dg-error ".E0740." "" { target *-*-* } .-2 }
}

fn main() {}

