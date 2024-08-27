#![crate_type = "lib"]

#[repr(uwu)] // { dg-error ".E0552." "" { target *-*-* } }
pub struct OwO;

#[repr(uwu = "a")] // { dg-error ".E0552." "" { target *-*-* } }
pub struct OwO2(i32);

#[repr(uwu(4))] // { dg-error ".E0552." "" { target *-*-* } }
pub struct OwO3 {
    x: i32,
}

#[repr(uwu, u8)] // { dg-error ".E0552." "" { target *-*-* } }
pub enum OwO4 {
    UwU = 1,
}

#[repr(uwu)] // { dg-error ".E0552." "" { target *-*-* } }
#[doc(owo)]  // { dg-error "" "" { target *-*-* } }
pub struct Owo5;

