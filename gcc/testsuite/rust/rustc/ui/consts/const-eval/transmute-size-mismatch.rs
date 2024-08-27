#![feature(core_intrinsics)]
#![feature(custom_mir)]

// These cases are statically rejected by `mem::transmute`, so we need custom
// MIR to be able to get to constant evaluation.
use std::intrinsics::mir::*;

#[custom_mir(dialect = "runtime", phase = "initial")]
const unsafe fn mir_transmute<T, U>(x: T) -> U {
    mir!{
        {
            RET = CastTransmute(x);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
            Return()
        }
    }
}

const FROM_BIGGER: u16 = unsafe { mir_transmute(123_i32) };

const FROM_SMALLER: u32 = unsafe { mir_transmute(123_i16) };

fn main() {}

