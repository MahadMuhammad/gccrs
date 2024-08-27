const a: &str = "foo";
const b: *const u8 = a as *const u8; // { dg-error ".E0606." "" { target *-*-* } }
const c: *const u8 = &a as *const u8; // { dg-error ".E0606." "" { target *-*-* } }

fn main() {
}

