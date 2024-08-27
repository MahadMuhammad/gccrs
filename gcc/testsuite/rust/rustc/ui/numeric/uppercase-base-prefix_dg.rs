//@ run-rustfix
// Checks that integers with an uppercase base prefix (0B, 0X, 0O) have a nice error
#![allow(unused_variables)]

fn main() {
    let a = 0XABCDEF;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let b = 0O755;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let c = 0B10101010;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let d = 0XABC_DEF;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let e = 0O7_55;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let f = 0B1010_1010;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let g = 0XABC_DEF_u64;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let h = 0O7_55_u32;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let i = 0B1010_1010_u8;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }
    //
    let j = 0XABCDEFu64;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let k = 0O755u32;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }

    let l = 0B10101010u8;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }
}

