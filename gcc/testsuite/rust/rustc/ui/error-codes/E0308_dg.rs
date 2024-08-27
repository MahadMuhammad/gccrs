#![feature(intrinsics)]
#![feature(rustc_attrs)]

extern "rust-intrinsic" {
    #[rustc_safe_intrinsic]
    fn size_of<T>(); // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {
}

