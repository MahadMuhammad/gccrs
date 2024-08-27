#![feature(rustc_attrs)]

macro_rules! check {
    ($expr: expr) => (
        #[rustc_dummy = $expr]
        use main as _;
    );
}

check!("0"); // OK
check!(0); // OK
check!(0u8); // { dg-error "" "" { target *-*-* } }
check!(-0); // { dg-error "" "" { target *-*-* } }
check!(0 + 0); // { dg-error "" "" { target *-*-* } }

fn main() {}

