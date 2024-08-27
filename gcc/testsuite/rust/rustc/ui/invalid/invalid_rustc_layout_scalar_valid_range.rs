#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_start(u32::MAX)] // { dg-error "" "" { target *-*-* } }
pub struct A(u32);

#[rustc_layout_scalar_valid_range_end(1, 2)] // { dg-error "" "" { target *-*-* } }
pub struct B(u8);

#[rustc_layout_scalar_valid_range_end(a = "a")] // { dg-error "" "" { target *-*-* } }
pub struct C(i32);

#[rustc_layout_scalar_valid_range_end(1)] // { dg-error "" "" { target *-*-* } }
enum E {
    X = 1,
    Y = 14,
}

#[rustc_layout_scalar_valid_range_start(rustc_layout_scalar_valid_range_start)] // { dg-error "" "" { target *-*-* } }
struct NonZero<T>(T);

fn not_field() -> impl Send {
    NonZero(false)
}

fn main() {
    let _ = A(0);
    let _ = B(0);
    let _ = C(0);
    unsafe {
        let _ = E::X;
    }
}

