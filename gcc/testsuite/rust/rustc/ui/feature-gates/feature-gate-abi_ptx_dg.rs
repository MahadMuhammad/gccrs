//@ needs-llvm-components: nvptx
//@ compile-flags: --target=nvptx64-nvidia-cuda --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]
#[lang="sized"]
trait Sized { }

extern "ptx-kernel" fn fu() {} // { dg-error ".E0658." "" { target *-*-* } }

trait T {
    extern "ptx-kernel" fn mu(); // { dg-error ".E0658." "" { target *-*-* } }
    extern "ptx-kernel" fn dmu() {} // { dg-error ".E0658." "" { target *-*-* } }
}

struct S;
impl T for S {
    extern "ptx-kernel" fn mu() {} // { dg-error ".E0658." "" { target *-*-* } }
}

impl S {
    extern "ptx-kernel" fn imu() {} // { dg-error ".E0658." "" { target *-*-* } }
}

type TAU = extern "ptx-kernel" fn(); // { dg-error ".E0658." "" { target *-*-* } }

extern "ptx-kernel" {} // { dg-error ".E0658." "" { target *-*-* } }

