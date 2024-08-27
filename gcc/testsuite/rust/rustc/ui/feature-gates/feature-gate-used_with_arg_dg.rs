#[used(linker)] // { dg-error ".E0658." "" { target *-*-* } }
static mut USED_LINKER: [usize; 1] = [0];

#[used(compiler)] // { dg-error ".E0658." "" { target *-*-* } }
static mut USED_COMPILER: [usize; 1] = [0];

fn main() {}

