//@ check-fail

pub fn main() { println!("🦀%%%", 0) } // { dg-error "" "" { target *-*-* } }

