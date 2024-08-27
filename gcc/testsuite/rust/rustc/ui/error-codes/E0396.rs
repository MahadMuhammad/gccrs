const REG_ADDR: *mut u8 = 0x5f3759df as *mut u8;

const VALUE: u8 = unsafe { *REG_ADDR };
// { dg-error ".E0658." "" { target *-*-* } .-1 }

const unsafe fn unreachable() -> ! {
    use std::convert::Infallible;

    const INFALLIBLE: *mut Infallible = &[] as *const [Infallible] as *const _ as _;
    match *INFALLIBLE {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

    const BAD: () = unsafe { match *INFALLIBLE {} };
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
}

fn main() {
}

