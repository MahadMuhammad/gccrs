#![feature(used_with_arg)]

#[used(linker)]
static mut USED_LINKER: [usize; 1] = [0];

#[used(compiler)]
static mut USED_COMPILER: [usize; 1] = [0];

#[used(compiler)] // { dg-error "" "" { target *-*-* } }
#[used(linker)]
static mut USED_COMPILER_LINKER2: [usize; 1] = [0];

#[used(compiler)] // { dg-error "" "" { target *-*-* } }
#[used(linker)]
#[used(compiler)]
#[used(linker)]
static mut USED_COMPILER_LINKER3: [usize; 1] = [0];

fn main() {}

