const FOO: &'static[u32] = &[1, 2, 3];
const BAR: u32 = FOO[5];
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

fn main() {
    let _ = BAR;
}

