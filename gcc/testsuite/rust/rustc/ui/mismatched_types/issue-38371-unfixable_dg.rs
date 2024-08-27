fn ugh(&[bar]: &u32) {} // { dg-error ".E0529." "" { target *-*-* } }

fn bgh(&&bar: u32) {} // { dg-error ".E0308." "" { target *-*-* } }

fn main() {}

