#[repr(C, Rust)] // { dg-error ".E0566." "" { target *-*-* } }
struct S {
    a: i32,
}


#[repr(Rust)] // { dg-error ".E0566." "" { target *-*-* } }
#[repr(C)]
struct T {
    a: i32,
}

#[repr(Rust, u64)] // { dg-error ".E0566." "" { target *-*-* } }
enum U {
    V,
}

#[repr(Rust, simd)]
// { dg-error ".E0566." "" { target *-*-* } .-1 }
// { dg-error ".E0566." "" { target *-*-* } .-2 }
struct F32x4(f32, f32, f32, f32);

fn main() {}

