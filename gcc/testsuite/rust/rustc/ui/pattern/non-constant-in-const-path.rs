// Checks if we emit `PatternError`s correctly.
// This is also a regression test for #27895 and #68394.

static FOO: u8 = 10;

fn main() {
    let x = 0;
    let 0u8..=x = 0;
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    let 0u8..=FOO = 0;
// { dg-error ".E0158." "" { target *-*-* } .-1 }
    match 1 {
        0 ..= x => {}
// { dg-error ".E0080." "" { target *-*-* } .-1 }
        0 ..= FOO => {}
// { dg-error ".E0158." "" { target *-*-* } .-1 }
    };
}

