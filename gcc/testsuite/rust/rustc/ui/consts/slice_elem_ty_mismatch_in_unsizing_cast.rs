const FOO: &str = unsafe { &*(1_usize as *const [i64; 0] as *const [u8] as *const str) };
// { dg-error ".E0607." "" { target *-*-* } .-1 }

fn main() {}

