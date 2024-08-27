use std::io::Read;

fn f<T: Read, U, Read>() {} // { dg-error ".E0404." "" { target *-*-* } }

fn main() {}

