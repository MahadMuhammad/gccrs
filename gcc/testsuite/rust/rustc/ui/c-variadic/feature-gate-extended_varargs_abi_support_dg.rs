fn efiapi(f: extern "efiapi" fn(usize, ...)) {
// { dg-error ".E0045." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    f(22, 44);
}
fn sysv(f: extern "sysv64" fn(usize, ...)) {
// { dg-error ".E0045." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    f(22, 44);
}
fn win(f: extern "win64" fn(usize, ...)) {
// { dg-error ".E0045." "" { target *-*-* } .-1 }
// { dg-error ".E0045." "" { target *-*-* } .-2 }
    f(22, 44);
}

fn main() {}

