// Regression test for issue 123710.
// Tests that the we do not ICE in KnownPanicsLint
// when a union contains an enum with an repr(packed),
// which is a repr not supported for enums

#[repr(packed)]
// { dg-error ".E0517." "" { target *-*-* } .-1 }
#[repr(u32)]
enum E {
    A,
    B,
    C,
}

fn main() {
    union InvalidTag {
        int: u32,
        e: E,
// { dg-error ".E0740." "" { target *-*-* } .-1 }
    }
    let _invalid_tag = InvalidTag { int: 4 };
}

