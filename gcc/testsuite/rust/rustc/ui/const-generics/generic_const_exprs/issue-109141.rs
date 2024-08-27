#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

impl EntriesBuffer {
    fn a(&self) -> impl Iterator {
        self.0.iter_mut() // { dg-error ".E0700." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

struct EntriesBuffer(Box<[[u8; HashesEntryLEN]; 5]>);
// { dg-error ".E0425." "" { target *-*-* } .-1 }

fn main() {}

