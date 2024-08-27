fn f(p: *const u8) -> u8 {
    let _ = *p; // { dg-error ".E0133." "" { target *-*-* } }
    let _: u8 = *p; // { dg-error ".E0133." "" { target *-*-* } }
    _ = *p; // { dg-error ".E0133." "" { target *-*-* } }
    return *p; // { dg-error ".E0133." "" { target *-*-* } }
}

fn main() {
}

