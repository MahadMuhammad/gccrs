#![feature(macro_metavar_expr)]

macro_rules! metavar {
    ( $i:expr ) => {
        ${len(0)}
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

const _: i32 = metavar!(0);

fn main() {}

