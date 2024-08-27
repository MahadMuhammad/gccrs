// Test that when adt_const_params is not enabled, we suggest adding the feature only when
// it would be possible for the type to be used as a const generic or when it's likely
// possible for the user to fix their type to be used.

// Can never be used as const generics.
fn uwu_0<const N: &'static mut ()>() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }

// Needs the feature but can be used, so suggest adding the feature.
fn owo_0<const N: &'static u32>() {}
// { dg-error "" "" { target *-*-* } .-1 }

// Can only be used in const generics with changes.
struct Meow {
    meow: u8,
}

fn meow_0<const N: Meow>() {}
// { dg-error "" "" { target *-*-* } .-1 }
fn meow_1<const N: &'static Meow>() {}
// { dg-error "" "" { target *-*-* } .-1 }
fn meow_2<const N: [Meow; 100]>() {}
// { dg-error "" "" { target *-*-* } .-1 }
fn meow_3<const N: (Meow, u8)>() {}
// { dg-error "" "" { target *-*-* } .-1 }

// This is suboptimal that it thinks it can be used
// but better to suggest the feature to the user.
fn meow_4<const N: (Meow, String)>() {}
// { dg-error "" "" { target *-*-* } .-1 }

// Non-local ADT that does not impl `ConstParamTy`
fn nya_0<const N: String>() {}
// { dg-error "" "" { target *-*-* } .-1 }
fn nya_1<const N: Vec<u32>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

