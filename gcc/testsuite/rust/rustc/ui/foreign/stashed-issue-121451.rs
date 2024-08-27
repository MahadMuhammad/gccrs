extern "C" fn _f() -> libc::uintptr_t {}
// { dg-error ".E0433." "" { target *-*-* } .-1 }

fn main() {}

