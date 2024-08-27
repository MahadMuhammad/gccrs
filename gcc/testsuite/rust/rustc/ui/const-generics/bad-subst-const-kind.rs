//@ incremental
#![crate_type = "lib"]

trait Q {
    const ASSOC: usize;
}

impl<const N: u64> Q for [u8; N] {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    const ASSOC: usize = 1;
}

pub fn test() -> [u8; <[u8; 13] as Q>::ASSOC] {
// { dg-error "" "" { target *-*-* } .-1 }
    todo!()
}

