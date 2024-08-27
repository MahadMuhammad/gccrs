#![feature(const_precise_live_drops)]
pub const fn id<T>(x: T) -> T { x }
pub const C: () = {
    let _: &'static _ = &id(&String::new());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
// { dg-error ".E0716." "" { target *-*-* } .-2 }
// { dg-error ".E0493." "" { target *-*-* } .-3 }
};

fn main() {
    let _: &'static _ = &id(&String::new());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
// { dg-error ".E0716." "" { target *-*-* } .-2 }
}

