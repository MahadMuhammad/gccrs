#![feature(intrinsics, rustc_attrs)]

extern "rust-intrinsic" {
    #[rustc_safe_intrinsic]
    fn size_of<T, U>() -> usize; // { dg-error ".E0094." "" { target *-*-* } }
}

fn main() {
}

