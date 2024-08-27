//@ check-pass

#![feature(adt_const_params, unsized_const_params, generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

pub struct Changes<const CHANGES: &'static [&'static str]>
where
    [(); CHANGES.len()]:,
{
    changes: [usize; CHANGES.len()],
}

impl<const CHANGES: &'static [&'static str]> Changes<CHANGES>
where
    [(); CHANGES.len()]:,
{
    pub const fn new() -> Self {
        Self { changes: [0; CHANGES.len()] }
    }
}

pub fn main() {}

