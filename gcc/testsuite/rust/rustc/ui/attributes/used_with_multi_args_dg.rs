#![feature(used_with_arg)]

#[used(compiler, linker)] // { dg-error "" "" { target *-*-* } }
static mut USED_COMPILER_LINKER: [usize; 1] = [0];

fn main() {}

