//@ normalize-stderr-test: "pref: Align\([1-8] bytes\)" -> "pref: $$SOME_ALIGN"
#![feature(never_type, rustc_attrs, type_alias_impl_trait, repr_simd)]
#![crate_type = "lib"]

#[rustc_layout(debug)]
#[derive(Copy, Clone)]
enum E { Foo, Bar(!, i32, i32) } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
struct S { f1: i32, f2: (), f3: i32 } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
union U { f1: (i32, i32), f3: i32 } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
type Test = Result<i32, i32>; // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
type T = impl std::fmt::Debug; // { dg-error "" "" { target *-*-* } }
fn f() -> T {
    0i32
}

#[rustc_layout(debug)]
pub union V { // { dg-error "" "" { target *-*-* } }
    a: [u16; 0],
    b: u8,
}

#[rustc_layout(debug)]
pub union W { // { dg-error "" "" { target *-*-* } }
    b: u8,
    a: [u16; 0],
}

#[rustc_layout(debug)]
pub union Y { // { dg-error "" "" { target *-*-* } }
    b: [u8; 0],
    a: [u16; 0],
}

#[rustc_layout(debug)]
#[repr(packed(1))]
union P1 { x: u32 } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
#[repr(packed(1))]
union P2 { x: (u32, u32) } // { dg-error "" "" { target *-*-* } }

#[repr(simd)]
#[derive(Copy, Clone)]
struct F32x4(f32, f32, f32, f32);

#[rustc_layout(debug)]
#[repr(packed(1))]
union P3 { x: F32x4 } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
#[repr(packed(1))]
union P4 { x: E } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
#[repr(packed(1))]
union P5 { zst: [u16; 0], byte: u8 } // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
type X = std::mem::MaybeUninit<u8>; // { dg-error "" "" { target *-*-* } }

#[rustc_layout(debug)]
const C: () = (); // { dg-error "" "" { target *-*-* } }

impl S {
    #[rustc_layout(debug)]
    const C: () = (); // { dg-error "" "" { target *-*-* } }
}

#[rustc_layout(debug)]
type Impossible = (str, str); // { dg-error ".E0277." "" { target *-*-* } }

