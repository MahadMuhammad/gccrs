#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn test<'a>(
    _: &'a (),
) -> [(); { // { dg-error ".E0308." "" { target *-*-* } }
    let x: &'a ();
// { dg-error "" "" { target *-*-* } .-1 }
    1
}] {
}

fn main() {}

