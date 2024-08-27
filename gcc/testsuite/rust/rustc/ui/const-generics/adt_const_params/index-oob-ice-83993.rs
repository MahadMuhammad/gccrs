// issue: rust-lang/rust/#83993

#![feature(adt_const_params)]

fn bug<'a>()
where
    for<'b> [(); {
        let x: &'b ();
// { dg-error "" "" { target *-*-* } .-1 }
        0
    }]:,
{
}

fn bad()
where
    for<'b> [(); {
        let _: &'b ();
// { dg-error "" "" { target *-*-* } .-1 }
        0
    }]: Sized,
{
}
fn good()
where
    for<'b> [(); { 0 }]: Sized,
{
}

pub fn main() {}

