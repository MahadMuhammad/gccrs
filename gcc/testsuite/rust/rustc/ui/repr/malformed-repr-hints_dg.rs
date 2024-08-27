// Regression test for various ICEs inspired by
// https://github.com/rust-lang/rust/issues/83921#issuecomment-814640734

//@ compile-flags: -Zdeduplicate-diagnostics=yes

#[repr(packed())]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
struct S1;

#[repr(align)]
// { dg-error ".E0589." "" { target *-*-* } .-1 }
struct S2;

#[repr(align(2, 4))]
// { dg-error ".E0693." "" { target *-*-* } .-1 }
struct S3;

#[repr(align())]
// { dg-error ".E0693." "" { target *-*-* } .-1 }
struct S4;

// Regression test for issue #118334:
#[repr(Rust(u8))]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
#[repr(Rust(0))]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
#[repr(Rust = 0)]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
struct S5;

#[repr(i8())]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
enum E1 { A, B }

#[repr(u32(42))]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
enum E2 { A, B }

#[repr(i64 = 2)]
// { dg-error ".E0552." "" { target *-*-* } .-1 }
enum E3 { A, B }

fn main() {}

