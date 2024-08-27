#![feature(core_intrinsics)]
#![feature(const_intrinsic_raw_eq)]

const RAW_EQ_PADDING: bool = unsafe {
    std::intrinsics::raw_eq(&(1_u8, 2_u16), &(1_u8, 2_u16))
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
};

const RAW_EQ_PTR: bool = unsafe {
    std::intrinsics::raw_eq(&(&0), &(&1))
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
};

pub fn main() {
}

