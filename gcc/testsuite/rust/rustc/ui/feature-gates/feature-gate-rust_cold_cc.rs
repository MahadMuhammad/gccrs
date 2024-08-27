#![crate_type = "lib"]

extern "rust-cold" fn fu() {} // { dg-error ".E0658." "" { target *-*-* } }

trait T {
    extern "rust-cold" fn mu(); // { dg-error ".E0658." "" { target *-*-* } }
    extern "rust-cold" fn dmu() {} // { dg-error ".E0658." "" { target *-*-* } }
}

struct S;
impl T for S {
    extern "rust-cold" fn mu() {} // { dg-error ".E0658." "" { target *-*-* } }
}

impl S {
    extern "rust-cold" fn imu() {} // { dg-error ".E0658." "" { target *-*-* } }
}

type TAU = extern "rust-cold" fn(); // { dg-error ".E0658." "" { target *-*-* } }

extern "rust-cold" {} // { dg-error ".E0658." "" { target *-*-* } }

