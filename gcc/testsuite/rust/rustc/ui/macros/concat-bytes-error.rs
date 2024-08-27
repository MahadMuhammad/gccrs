#![feature(concat_bytes)]

fn main() {
    concat_bytes!(pie); // { dg-error "" "" { target *-*-* } }
    concat_bytes!(pie, pie); // { dg-error "" "" { target *-*-* } }
    concat_bytes!("tnrsi", "tnri"); // { dg-error "" "" { target *-*-* } }
    concat_bytes!(2.8); // { dg-error "" "" { target *-*-* } }
    concat_bytes!(300); // { dg-error "" "" { target *-*-* } }
    concat_bytes!('a'); // { dg-error "" "" { target *-*-* } }
    concat_bytes!(true, false); // { dg-error "" "" { target *-*-* } }
    concat_bytes!(42, b"va", b'l'); // { dg-error "" "" { target *-*-* } }
    concat_bytes!(42, b"va", b'l', [1, 2]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([
        "hi", // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        'a', // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        true, // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        false, // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        2.6, // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        265, // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        -33, // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        b"hi!", // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!([
        [5, 6, 7], // { dg-error "" "" { target *-*-* } }
    ]);
    concat_bytes!(5u16); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([5u16]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([3; ()]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([3; -2]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([pie; -2]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([pie; 2]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([2.2; 0]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([5.5; ()]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([[1, 2, 3]; 3]); // { dg-error "" "" { target *-*-* } }
    concat_bytes!([[42; 2]; 3]); // { dg-error "" "" { target *-*-* } }
}

