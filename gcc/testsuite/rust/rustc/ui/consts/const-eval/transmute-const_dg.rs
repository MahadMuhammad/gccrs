//@ stderr-per-bitwidth
use std::mem;

static FOO: bool = unsafe { mem::transmute(3u8) };
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

