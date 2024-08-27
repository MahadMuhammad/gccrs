#![deny(dead_code)]
// { dg-note "" "" { target *-*-* } .-1 }

use std::marker::PhantomData;

const LEN: usize = 4;

struct UnusedAtTheEnd(i32, f32, [u8; LEN], String, u8);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

struct UnusedJustOneField(i32);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

struct UnusedInTheMiddle(i32, f32, String, u8, u32);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

struct GoodUnit(());

struct GoodPhantom(PhantomData<i32>);

struct Void;
struct GoodVoid(Void);

fn main() {
    let u1 = UnusedAtTheEnd(42, 3.14, [0, 1, 2, 3], "def".to_string(), 4u8);
    let _ = u1.0;

    let _ = UnusedJustOneField(42);

    let u2 = UnusedInTheMiddle(42, 3.14, "def".to_string(), 4u8, 5);
    let _ = u2.0;
    let _ = u2.3;


    let gu = GoodUnit(());
    let gp = GoodPhantom(PhantomData);
    let gv = GoodVoid(Void);

    let _ = (gu, gp, gv);
}

