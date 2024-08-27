#![feature(const_mut_refs)]

enum E {
    A(u8),
    B,
}

const _: u8 = {
    let mut e = E::A(1);
    let p = if let E::A(x) = &mut e { x as *mut u8 } else { unreachable!() };
    // Make sure overwriting `e` uninitializes other bytes
    e = E::B;
    unsafe { *p }
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
};

fn main() {}

