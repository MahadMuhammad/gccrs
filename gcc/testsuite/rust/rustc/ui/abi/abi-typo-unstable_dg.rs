// rust-intrinsic is unstable and not enabled, so it should not be suggested as a fix
extern "rust-intrinsec" fn rust_intrinsic() {} // { dg-error ".E0703." "" { target *-*-* } }

fn main() {
    rust_intrinsic();
}

